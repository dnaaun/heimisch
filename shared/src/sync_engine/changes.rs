use std::collections::hash_map::Entry;
use std::collections::HashMap;
use typesafe_idb::{ReadOnly, Store};

use typesafe_idb::{Present, StoreMarker, TxnMode};

use crate::avail::{MergeError, MergeStructWithAvails};
use crate::types::issue_comment::{IssueComment, IssueCommentId};
use crate::types::label::{Label, LabelId};

use super::optimistic::db_with_optimistic_changes::{
    DbWithOptimisticChanges, TxnBuilderWithOptimisticChanges, TxnWithOptimisticChanges,
};
use super::TypedTransportTrait;
use super::{
    super::types::{
        github_app::{GithubApp, GithubAppId},
        issue::{Issue, IssueId},
        license::{License, LicenseId},
        milestone::{Milestone, MilestoneId},
        repository::{Repository, RepositoryId},
        user::{User, UserId},
    },
    error::SyncResult,
    SyncEngine,
};

#[derive(Debug, Clone)]
pub struct Deleted<S: Store>(pub S::Id);

#[derive(Debug, Clone)]
pub enum ExistingOrDeleted<S: Store> {
    Existing(S),
    Deleted(S::Id),
}

impl<S: Store + MergeStructWithAvails> MergeStructWithAvails for ExistingOrDeleted<S> {
    fn merge(&mut self, other: Self) -> Result<(), MergeError> {
        use ExistingOrDeleted::*;
        match (self, other) {
            (Existing(this), Existing(other)) => this.merge(other),
            (this @ Deleted(_), other @ Existing(_)) | (this @ Existing(_), other @ Deleted(_)) => {
                *this = other;
                Ok(())
            }
            (Deleted(_), Deleted(_)) => Ok(()),
        }
    }

    fn with_merged(self, other: Self) -> Result<Self, MergeError> {
        use ExistingOrDeleted::*;
        match (self, other) {
            (Existing(this), Existing(other)) => Ok(Existing(this.with_merged(other)?)),
            (Existing(_), other @ Deleted(_)) | (Deleted(_), other @ Existing(_)) => Ok(other),
            (this @ Deleted(_), Deleted(_)) => Ok(this),
        }
    }
}

impl<S: Store> From<S> for ExistingOrDeleted<S> {
    fn from(value: S) -> Self {
        Self::Existing(value)
    }
}

impl<S: Store> From<Deleted<S>> for ExistingOrDeleted<S> {
    fn from(value: Deleted<S>) -> Self {
        Self::Deleted(value.0)
    }
}

impl<S> AddChanges<ExistingOrDeleted<S>> for Changes
where
    S: Store,
    Changes: AddChanges<Deleted<S>>,
    Changes: AddChanges<S>,
{
    fn add(&mut self, change: ExistingOrDeleted<S>) -> Result<&mut Self, MergeError> {
        match change {
            ExistingOrDeleted::Existing(item) => self.add(item),
            ExistingOrDeleted::Deleted(id) => self.add(Deleted::<S>(id)),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Changes {
    pub github_apps: HashMap<GithubAppId, ExistingOrDeleted<GithubApp>>,
    pub issues: HashMap<IssueId, ExistingOrDeleted<Issue>>,
    pub issue_comments: HashMap<IssueCommentId, ExistingOrDeleted<IssueComment>>,
    pub users: HashMap<UserId, ExistingOrDeleted<User>>,
    pub repositorys: HashMap<RepositoryId, ExistingOrDeleted<Repository>>,
    pub licenses: HashMap<LicenseId, ExistingOrDeleted<License>>,
    pub milestones: HashMap<MilestoneId, ExistingOrDeleted<Milestone>>,
    pub labels: HashMap<LabelId, ExistingOrDeleted<Label>>,
}

pub trait IntoChanges {
    fn into_changes(self) -> Result<Changes, MergeError>;
}

impl<T> IntoChanges for T
where
    Changes: AddChanges<T>,
{
    fn into_changes(self) -> Result<Changes, MergeError> {
        let mut changes = Changes::default();
        changes.add(self)?;
        Ok(changes)
    }
}

pub trait StoreMarkersForChanges:
    StoreMarker<Milestone>
    + StoreMarker<Label>
    + StoreMarker<License>
    + StoreMarker<Repository>
    + StoreMarker<User>
    + StoreMarker<Issue>
    + StoreMarker<IssueComment>
    + StoreMarker<GithubApp>
    + Default
{
}

impl<T> StoreMarkersForChanges for T where
    T: StoreMarker<Milestone>
        + StoreMarker<License>
        + StoreMarker<Label>
        + StoreMarker<Repository>
        + StoreMarker<User>
        + StoreMarker<Issue>
        + StoreMarker<IssueComment>
        + StoreMarker<GithubApp>
        + Default
{
}

impl Changes {
    pub fn with_added(mut self, changes: Changes) -> Result<Self, MergeError> {
        self.add(changes)?;
        Ok(self)
    }

    /// A transaction builder that contains all the stores that `Changes` might interact with.
    pub fn txn<DbStoreMarkers>(
        db: &DbWithOptimisticChanges<DbStoreMarkers>,
    ) -> TxnBuilderWithOptimisticChanges<'_, DbStoreMarkers, impl StoreMarkersForChanges, ReadOnly>
    where
        DbStoreMarkers: StoreMarkersForChanges,
    {
        db.txn()
            .with_store::<GithubApp>()
            .with_store::<Issue>()
            .with_store::<Label>()
            .with_store::<IssueComment>()
            .with_store::<User>()
            .with_store::<Repository>()
            .with_store::<License>()
            .with_store::<Milestone>()
    }

    /// Add some iterable of changes.
    pub fn add_iter<A>(&mut self, iter: impl IntoIterator<Item = A>) -> Result<(), MergeError>
    where
        Changes: AddChanges<A>,
    {
        for change in iter {
            self.add(change)?;
        }
        Ok(())
    }

    pub fn try_from_iter<A>(iter: impl IntoIterator<Item = A>) -> Result<Self, MergeError>
    where
        Changes: AddChanges<A>,
    {
        let mut changes = Self::default();
        changes.add_iter(iter)?;
        Ok(changes)
    }

    pub fn try_add_iter<A, E>(
        &mut self,
        iter: impl IntoIterator<Item = Result<A, E>>,
    ) -> Result<Result<(), MergeError>, E>
    where
        Changes: AddChanges<A>,
    {
        for change in iter {
            match self.add(change?) {
                Ok(_) => (),
                Err(err) => return Ok(Err(err)),
            }
        }
        Ok(Ok(()))
    }

    /// Just trying to make life easier for myself is all: I do often collect changes from iterators
    /// whose items are results.
    pub fn try_try_from_iter<A, E>(
        iter: impl IntoIterator<Item = Result<A, E>>,
    ) -> Result<Result<Self, MergeError>, E>
    where
        Changes: AddChanges<A>,
    {
        let mut changes = Self::default();
        match changes.try_add_iter(iter)? {
            Ok(_) => (),
            Err(err) => return Ok(Err(err)),
        };

        Ok(Ok(changes))
    }
}

pub trait AddChanges<T> {
    fn add(&mut self, change: T) -> Result<&mut Self, MergeError>;
}

impl AddChanges<Deleted<GithubApp>> for Changes {
    fn add(&mut self, change: Deleted<GithubApp>) -> Result<&mut Self, MergeError> {
        self.github_apps.insert(change.0, change.into());
        Ok(self)
    }
}

impl AddChanges<GithubApp> for Changes {
    fn add(&mut self, change: GithubApp) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.github_apps.entry(change.id) {
            e.insert(change.into());
        } else {
            let cur = self.github_apps.get_mut(&change.id).expect("");
            cur.merge(change.into())?;
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<User>> for Changes {
    fn add(&mut self, change: Deleted<User>) -> Result<&mut Self, MergeError> {
        self.users.insert(change.0, change.into());
        Ok(self)
    }
}

impl AddChanges<User> for Changes {
    fn add(&mut self, change: User) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.users.entry(change.id) {
            e.insert(change.into());
        } else {
            let cur = self.users.get_mut(&change.id).expect("");
            cur.merge(change.into())?;
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<Issue>> for Changes {
    fn add(&mut self, change: Deleted<Issue>) -> Result<&mut Self, MergeError> {
        self.issues.insert(change.0, change.into());
        Ok(self)
    }
}

impl AddChanges<Issue> for Changes {
    fn add(&mut self, change: Issue) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.issues.entry(change.id) {
            e.insert(change.into());
        } else {
            let cur = self.issues.get_mut(&change.id).expect("");
            cur.merge(change.into())?;
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<IssueComment>> for Changes {
    fn add(&mut self, change: Deleted<IssueComment>) -> Result<&mut Self, MergeError> {
        self.issue_comments.insert(change.0, change.into());
        Ok(self)
    }
}
impl AddChanges<IssueComment> for Changes {
    fn add(&mut self, change: IssueComment) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.issue_comments.entry(change.id) {
            e.insert(change.into());
        } else {
            let cur = self.issue_comments.get_mut(&change.id).expect("");
            cur.merge(change.into())?;
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<Repository>> for Changes {
    fn add(&mut self, change: Deleted<Repository>) -> Result<&mut Self, MergeError> {
        self.repositorys.insert(change.0, change.into());
        Ok(self)
    }
}
impl AddChanges<Repository> for Changes {
    fn add(&mut self, change: Repository) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.repositorys.entry(change.id) {
            e.insert(change.into());
        } else {
            let cur = self.repositorys.get_mut(&change.id).expect("");
            cur.merge(change.into())?;
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<License>> for Changes {
    fn add(&mut self, change: Deleted<License>) -> Result<&mut Self, MergeError> {
        self.licenses.insert(change.0.clone(), change.into());
        Ok(self)
    }
}
impl AddChanges<License> for Changes {
    fn add(&mut self, change: License) -> Result<&mut Self, MergeError> {
        if self.licenses.contains_key(&change.key) {
            let cur = self.licenses.get_mut(&change.key).expect("");
            cur.merge(change.into())?;
        } else {
            self.licenses.insert(change.key.clone(), change.into());
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<Milestone>> for Changes {
    fn add(&mut self, change: Deleted<Milestone>) -> Result<&mut Self, MergeError> {
        self.milestones.insert(change.0, change.into());
        Ok(self)
    }
}
impl AddChanges<Milestone> for Changes {
    fn add(&mut self, change: Milestone) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.milestones.entry(change.id) {
            e.insert(change.into());
        } else {
            let cur = self.milestones.get_mut(&change.id).expect("");
            cur.merge(change.into())?;
        }
        Ok(self)
    }
}

impl AddChanges<Deleted<Label>> for Changes {
    fn add(&mut self, change: Deleted<Label>) -> Result<&mut Self, MergeError> {
        self.labels.insert(change.0, change.into());
        Ok(self)
    }
}
impl AddChanges<Label> for Changes {
    fn add(&mut self, change: Label) -> Result<&mut Self, MergeError> {
        self.labels.insert(change.id, change.into());
        Ok(self)
    }
}

impl AddChanges<Changes> for Changes {
    fn add(&mut self, other: Changes) -> Result<&mut Self, MergeError> {
        let Changes {
            github_apps,
            issues,
            issue_comments,
            users,
            repositorys,
            licenses,
            milestones,
            labels,
        } = other;

        for (_, github_app) in github_apps {
            self.add(github_app)?;
        }
        for (_, issue) in issues {
            self.add(issue)?;
        }
        for (_, issue_comment) in issue_comments {
            self.add(issue_comment)?;
        }

        for (_, user) in users {
            self.add(user)?;
        }
        for (_, repository) in repositorys {
            self.add(repository)?;
        }
        for (_, license) in licenses {
            self.add(license)?;
        }

        for (_, label) in labels {
            self.add(label)?;
        }
        for (_, milestone) in milestones {
            self.add(milestone)?;
        }
        Ok(self)
    }
}

impl<T> AddChanges<Option<T>> for Changes
where
    Changes: AddChanges<T>,
{
    fn add(&mut self, change: Option<T>) -> Result<&mut Self, MergeError> {
        match change {
            Some(t) => self.add(t),
            None => Ok(self),
        }
    }
}

impl<T> AddChanges<Vec<T>> for Changes
where
    Changes: AddChanges<T>,
{
    fn add(&mut self, changes: Vec<T>) -> Result<&mut Self, MergeError> {
        for change in changes {
            self.add(change)?;
        }
        Ok(self)
    }
}

impl<W: TypedTransportTrait> SyncEngine<W> {
    pub async fn persist_changes<
        Marker: StoreMarkersForChanges,
        Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
    >(
        &self,
        txn: &TxnWithOptimisticChanges<Marker, Mode>,
        changes: impl IntoChanges,
    ) -> SyncResult<(), W> {
        let Changes {
            github_apps,
            issues,
            issue_comments,
            users,
            repositorys,
            licenses,
            labels,
            milestones,
        } = changes.into_changes()?;
        persist_changes_to_issues::<W, Marker, Mode>(txn, issues).await?;
        persist_changes_to_issue_comments::<W, Marker, Mode>(txn, issue_comments).await?;
        persist_changes_to_github_apps::<W, Marker, Mode>(txn, github_apps).await?;
        persist_changes_to_users::<W, Marker, Mode>(txn, users).await?;
        persist_changes_to_repositorys::<W, Marker, Mode>(txn, repositorys).await?;
        persist_changes_to_milestones::<W, Marker, Mode>(txn, milestones).await?;
        persist_changes_to_licenses::<W, Marker, Mode>(txn, licenses).await?;
        upsert_labels::<W, Marker, Mode>(txn, labels).await?;

        Ok(())
    }
}

async fn persist_changes_to_issues<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    issues: HashMap<IssueId, ExistingOrDeleted<Issue>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Issue>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let issue_store = txn.object_store::<Issue>()?;
    for (_, issue) in issues {
        match issue {
            ExistingOrDeleted::Existing(issue) => {
                let existing = issue_store.get(&issue.id).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(issue)?,
                    None => issue,
                };
                issue_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                issue_store.delete(&id).await?;
            }
        }
    }

    Ok(())
}

async fn persist_changes_to_issue_comments<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    issue_comments: HashMap<IssueCommentId, ExistingOrDeleted<IssueComment>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<IssueComment>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let issue_comment_store = txn.object_store::<IssueComment>()?;
    for (_, issue_comment) in issue_comments {
        match issue_comment {
            ExistingOrDeleted::Existing(issue_comment) => {
                let existing = issue_comment_store.get(&issue_comment.id).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(issue_comment)?,
                    None => issue_comment,
                };
                issue_comment_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                issue_comment_store.delete(&id).await?;
            }
        }
    }
    Ok(())
}

async fn persist_changes_to_github_apps<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    github_apps: HashMap<GithubAppId, ExistingOrDeleted<GithubApp>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<GithubApp>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let github_app_store = txn.object_store::<GithubApp>()?;
    for (_, github_app) in github_apps {
        match github_app {
            ExistingOrDeleted::Existing(github_app) => {
                let existing = github_app_store.get(&github_app.id).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(github_app)?,
                    None => github_app,
                };
                github_app_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                github_app_store.delete(&id).await?;
            }
        }
    }
    Ok(())
}

async fn persist_changes_to_users<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    users: HashMap<UserId, ExistingOrDeleted<User>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<User>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let user_store = txn.object_store::<User>()?;
    for (_, user) in users {
        match user {
            ExistingOrDeleted::Existing(user) => {
                let existing = user_store.get(&user.id).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(user)?,
                    None => user,
                };
                user_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                user_store.delete(&id).await?;
            }
        }
    }

    Ok(())
}

async fn persist_changes_to_licenses<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    licenses: HashMap<LicenseId, ExistingOrDeleted<License>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<License>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let license_store = txn.object_store::<License>()?;
    for (_, license) in licenses {
        match license {
            ExistingOrDeleted::Existing(license) => {
                let existing = license_store.get(&license.key).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(license)?,
                    None => license,
                };
                license_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                license_store.delete(&id).await?;
            }
        }
    }

    Ok(())
}

async fn persist_changes_to_milestones<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    milestones: HashMap<MilestoneId, ExistingOrDeleted<Milestone>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Milestone>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let milestone_store = txn.object_store::<Milestone>()?;
    for (_, milestone) in milestones {
        match milestone {
            ExistingOrDeleted::Existing(milestone) => {
                let existing = milestone_store.get(&milestone.id).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(milestone)?,
                    None => milestone,
                };
                milestone_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                milestone_store.delete(&id).await?;
            }
        }
    }

    Ok(())
}

async fn persist_changes_to_repositorys<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    repositorys: HashMap<RepositoryId, ExistingOrDeleted<Repository>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Repository>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let repository_store = txn.object_store::<Repository>()?;
    for (_, repository) in repositorys {
        match repository {
            ExistingOrDeleted::Existing(repository) => {
                let existing = repository_store.get(&repository.id).await?;
                let merged = match existing {
                    Some(existing) => existing.with_merged(repository)?,
                    None => repository,
                };
                repository_store.put(&merged).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                repository_store.delete(&id).await?;
            }
        }
    }

    Ok(())
}

async fn upsert_labels<W: TypedTransportTrait, Marker, Mode>(
    txn: &TxnWithOptimisticChanges<Marker, Mode>,
    labels: HashMap<LabelId, ExistingOrDeleted<Label>>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Label>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let label_store = txn.object_store::<Label>()?;
    for (_, label) in labels {
        match label {
            ExistingOrDeleted::Existing(label) => {
                label_store.put(&label).await?;
            }
            ExistingOrDeleted::Deleted(id) => {
                label_store.delete(&id).await?;
            }
        }
    }
    Ok(())
}

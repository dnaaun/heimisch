use std::collections::hash_map::Entry;
use std::collections::HashMap;
use typesafe_idb::{ReadOnly, TypesafeDb};

use typesafe_idb::{Present, StoreMarker, Txn, TxnBuilder, TxnMode};

use crate::avail::MergeError;
use crate::types::issue_comment::{IssueComment, IssueCommentId};
use crate::types::label::{Label, LabelId};

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

#[derive(Default, Debug, Clone)]
pub struct Changes {
    github_apps: HashMap<GithubAppId, GithubApp>,
    issues: HashMap<IssueId, Issue>,
    issue_comments: HashMap<IssueCommentId, IssueComment>,
    users: HashMap<UserId, User>,
    repositorys: HashMap<RepositoryId, Repository>,
    licenses: HashMap<LicenseId, License>,
    milestones: HashMap<MilestoneId, Milestone>,
    labels: HashMap<LabelId, Label>,
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
{
}

impl Changes {
    pub fn with_added(mut self, changes: Changes) -> Result<Self, MergeError> {
        self.add(changes)?;
        Ok(self)
    }

    /// A transaction builder that contains all the stores that `Changes` might interact with.
    pub fn txn<DbStoreMarkers>(
        db: &TypesafeDb<DbStoreMarkers>,
    ) -> TxnBuilder<'_, DbStoreMarkers, impl StoreMarkersForChanges, ReadOnly>
    where
        DbStoreMarkers: StoreMarkersForChanges,
    {
        Txn::builder(db)
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

impl AddChanges<GithubApp> for Changes {
    fn add(&mut self, change: GithubApp) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.github_apps.entry(change.id) {
            e.insert(change);
        } else {
            let cur = self.github_apps.get_mut(&change.id).expect("");
            cur.merge(change)?;
        }
        Ok(self)
    }
}

impl AddChanges<User> for Changes {
    fn add(&mut self, change: User) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.users.entry(change.id) {
            e.insert(change);
        } else {
            let cur = self.users.get_mut(&change.id).expect("");
            cur.merge(change)?;
        }
        Ok(self)
    }
}

impl AddChanges<Issue> for Changes {
    fn add(&mut self, change: Issue) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.issues.entry(change.id) {
            e.insert(change);
        } else {
            let cur = self.issues.get_mut(&change.id).expect("");
            cur.merge(change)?;
        }
        Ok(self)
    }
}

impl AddChanges<IssueComment> for Changes {
    fn add(&mut self, change: IssueComment) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.issue_comments.entry(change.id) {
            e.insert(change);
        } else {
            let cur = self.issue_comments.get_mut(&change.id).expect("");
            cur.merge(change)?;
        }
        Ok(self)
    }
}

impl AddChanges<Repository> for Changes {
    fn add(&mut self, change: Repository) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.repositorys.entry(change.id) {
            e.insert(change);
        } else {
            let cur = self.repositorys.get_mut(&change.id).expect("");
            cur.merge(change)?;
        }
        Ok(self)
    }
}

impl AddChanges<License> for Changes {
    fn add(&mut self, change: License) -> Result<&mut Self, MergeError> {
        if self.licenses.contains_key(&change.key) {
            let cur = self.licenses.get_mut(&change.key).expect("");
            cur.merge(change)?;
        } else {
            self.licenses.insert(change.key.clone(), change);
        }
        Ok(self)
    }
}

impl AddChanges<Milestone> for Changes {
    fn add(&mut self, change: Milestone) -> Result<&mut Self, MergeError> {
        if let Entry::Vacant(e) = self.milestones.entry(change.id) {
            e.insert(change);
        } else {
            let cur = self.milestones.get_mut(&change.id).expect("");
            cur.merge(change)?;
        }
        Ok(self)
    }
}

impl AddChanges<Label> for Changes {
    fn add(&mut self, change: Label) -> Result<&mut Self, MergeError> {
        self.labels.insert(change.id, change);
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
    pub async fn merge_and_upsert_changes<
        Marker: StoreMarkersForChanges,
        Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
    >(
        &self,
        txn: &Txn<Marker, Mode>,
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
        merge_and_upsert_issues::<W, Marker, Mode>(txn, issues).await?;
        merge_and_upsert_issue_comments::<W, Marker, Mode>(txn, issue_comments).await?;
        merge_and_upsert_github_apps::<W, Marker, Mode>(txn, github_apps).await?;
        merge_and_upsert_users::<W, Marker, Mode>(txn, users).await?;
        merge_and_upsert_repositorys::<W, Marker, Mode>(txn, repositorys).await?;
        merge_and_upsert_milestones::<W, Marker, Mode>(txn, milestones).await?;
        merge_and_upsert_licenses::<W, Marker, Mode>(txn, licenses).await?;
        upsert_labels::<W, Marker, Mode>(txn, labels).await?;

        Ok(())
    }
}

async fn merge_and_upsert_issues<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    issues: HashMap<IssueId, Issue>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Issue>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let issue_store = txn.object_store::<Issue>()?;
    for (_, issue) in issues {
        let existing = issue_store.get(&issue.id).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(issue)?,
            None => issue,
        };
        issue_store.put(&merged).await?;
    }

    Ok(())
}

async fn merge_and_upsert_issue_comments<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    issue_comments: HashMap<IssueCommentId, IssueComment>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<IssueComment>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let issue_comment_store = txn.object_store::<IssueComment>()?;
    for (_, issue_comment) in issue_comments {
        let existing = issue_comment_store.get(&issue_comment.id).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(issue_comment)?,
            None => issue_comment,
        };
        issue_comment_store.put(&merged).await?;
    }
    Ok(())
}

async fn merge_and_upsert_github_apps<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    github_apps: HashMap<GithubAppId, GithubApp>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<GithubApp>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let github_app_store = txn.object_store::<GithubApp>()?;
    for (_, github_app) in github_apps {
        let existing = github_app_store.get(&github_app.id).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(github_app)?,
            None => github_app,
        };
        github_app_store.put(&merged).await?;
    }
    Ok(())
}

async fn merge_and_upsert_users<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    users: HashMap<UserId, User>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<User>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let user_store = txn.object_store::<User>()?;
    for (_, user) in users {
        let existing = user_store.get(&user.id).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(user)?,
            None => user,
        };
        user_store.put(&merged).await?;
    }

    Ok(())
}

async fn merge_and_upsert_licenses<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    licenses: HashMap<LicenseId, License>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<License>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let license_store = txn.object_store::<License>()?;
    for (_, license) in licenses {
        let existing = license_store.get(&license.key).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(license)?,
            None => license,
        };
        license_store.put(&merged).await?;
    }

    Ok(())
}

async fn merge_and_upsert_milestones<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    milestones: HashMap<MilestoneId, Milestone>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Milestone>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let milestone_store = txn.object_store::<Milestone>()?;
    for (_, milestone) in milestones {
        let existing = milestone_store.get(&milestone.id).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(milestone)?,
            None => milestone,
        };
        milestone_store.put(&merged).await?;
    }

    Ok(())
}

async fn merge_and_upsert_repositorys<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    repositorys: HashMap<RepositoryId, Repository>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Repository>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let repository_store = txn.object_store::<Repository>()?;
    for (_, repository) in repositorys {
        let existing = repository_store.get(&repository.id).await?;
        let merged = match existing {
            Some(existing) => existing.with_merged(repository)?,
            None => repository,
        };
        repository_store.put(&merged).await?;
    }

    Ok(())
}

async fn upsert_labels<W: TypedTransportTrait, Marker, Mode>(
    txn: &Txn<Marker, Mode>,
    labels: HashMap<LabelId, Label>,
) -> SyncResult<(), W>
where
    Marker: StoreMarker<Label>,
    Mode: TxnMode<SupportsReadOnly = Present, SupportsReadWrite = Present>,
{
    let label_store = txn.object_store::<Label>()?;
    for (_, label) in labels {
        label_store.put(&label).await?;
    }
    Ok(())
}

use futures::future::join_all;

use super::{
    changes::Changes,
    conversions::from_issue_comment::from_issue_comment,
    error::{SyncErrorSrc, SyncResult},
    SyncEngine, WSClient, MAX_PER_PAGE,
};
use crate::types::{
    installation::InstallationId,
    issue::{Issue, NumberIndex},
    issue_comment::{IssueComment, RepositoryIdIndex},
    issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
    issues_initial_sync_status::InitialSyncStatusEnum,
    repository::{Repository, RepositoryId},
    user::User,
};

impl<W: WSClient> SyncEngine<W> {
    /// This function will try to find issue ids in the db by using the issue number in `issue_url`
    /// of issue_comment`.
    pub async fn ensure_initial_sync_issue_comments(
        &self,
        id: &RepositoryId,
        installation_id: &InstallationId,
    ) -> SyncResult<(), W::Error> {
        let mut page = 1;
        let txn = self
            .db
            .txn()
            .with_store::<IssueCommentsInitialSyncStatus>()
            .with_store::<IssueComment>()
            .build();
        let initial_sync_status = txn
            .object_store::<IssueCommentsInitialSyncStatus>()?
            .get(id)
            .await?;
        if let Some(initial_sync_status) = initial_sync_status {
            match initial_sync_status.status {
                InitialSyncStatusEnum::Full => return Ok(()),
                InitialSyncStatusEnum::Partial => {
                    page = (txn
                        .object_store::<IssueComment>()?
                        .index::<RepositoryIdIndex>()?
                        .get_all(Some(id))
                        .await?
                        .len() as f64
                        / f64::from(MAX_PER_PAGE))
                    .ceil() as i32;
                }
                InitialSyncStatusEnum::NoSync => (),
            }
        }
        drop(txn);
        let conf = self.get_api_conf(installation_id).await?;

        let txn = self
            .db
            .txn()
            .with_store::<Repository>()
            .with_store::<User>()
            .build();
        let repo = txn
            .object_store::<Repository>()?
            .get(id)
            .await?
            .ok_or_else(|| {
                SyncErrorSrc::DataModel(format!("repository with id {id:?}: doesn't exist"))
            })?;
        let repo_owner_id = repo.owner_id.ok_or_else(|| {
            SyncErrorSrc::DataModel(format!(
                "repository with id {id:?} has no owner id available"
            ))
        })?;
        let repo_owner = txn
            .object_store::<User>()?
            .get(&repo_owner_id)
            .await?
            .ok_or_else(|| {
                SyncErrorSrc::DataModel(format!("user with id {repo_owner_id:?}: doesn't exist"))
            })?;
        let owner_name = repo_owner.login;
        drop(txn);
        let repo_name = repo.name;

        // NOTE: Maybe abstract away dealing with pagination.
        loop {
            let issue_comments = github_api::apis::issues_api::issues_slash_list_comments_for_repo(
                &conf,
                &owner_name,
                &repo_name,
                "created".into(),
                "asc".into(),
                None,
                MAX_PER_PAGE.into(),
                page.into(),
            )
            .await?;
            let last_fetched_num = issue_comments.len();

            let db = self.db.clone();
            let issue_id_from_number = |number| {
                let id = *id;
                let txn = db.txn().with_store::<Issue>().build();
                async move {
                    txn.object_store::<Issue>()
                        .unwrap()
                        .index::<NumberIndex>()
                        .unwrap()
                        .get_all(Some(&number))
                        .await
                        .unwrap()
                        .into_iter()
                        .find(|issue| issue.repository_id == id)
                        .map(|i| i.id)
                }
            };
            let issue_comment_ids_and_changes = join_all(
                issue_comments
                    .into_iter()
                    .map(|r| from_issue_comment(issue_id_from_number, r, id)),
            )
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
            let changes =
                Changes::try_from_iter(issue_comment_ids_and_changes.into_iter().map(|r| r.1))?;

            let txn = Changes::txn(&self.db)
                .with_store::<IssueCommentsInitialSyncStatus>()
                .read_write()
                .build();
            txn.object_store::<IssueCommentsInitialSyncStatus>()?
                .put(&IssueCommentsInitialSyncStatus {
                    status: InitialSyncStatusEnum::Partial,
                    id: *id,
                })
                .await?;
            self.merge_and_upsert_changes(txn, changes).await?;

            page += 1;
            if last_fetched_num < MAX_PER_PAGE as usize {
                break;
            }
        }

        let txn = self
            .db
            .txn()
            .with_store::<IssueCommentsInitialSyncStatus>()
            .read_write()
            .build();
        txn.object_store::<IssueCommentsInitialSyncStatus>()?
            .put(&IssueCommentsInitialSyncStatus {
                status: InitialSyncStatusEnum::Full,
                id: *id,
            })
            .await?;
        Ok(())
    }
}

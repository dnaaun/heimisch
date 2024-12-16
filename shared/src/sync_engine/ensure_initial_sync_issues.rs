use super::{
    changes::Changes,
    conversions::from_issue::from_issue,
    error::{SyncErrorSrc, SyncResult},
    SyncEngine, WSClient, MAX_PER_PAGE,
};
use crate::types::{
    installation::InstallationId,
    issue::{Issue, RepositoryIdIndex},
    issues_initial_sync_status::{InitialSyncStatusEnum, IssuesInitialSyncStatus},
    repository::{Repository, RepositoryId},
    user::User,
};

impl<W: WSClient> SyncEngine<W> {
    pub async fn ensure_initial_sync_issues(
        &self,
        id: &RepositoryId,
        installation_id: &InstallationId,
    ) -> SyncResult<(), W::Error> {
        let mut page = 1;
        let txn = self
            .db
            .txn()
            .with_store::<IssuesInitialSyncStatus>()
            .with_store::<Issue>()
            .ro();
        let initial_sync_status = txn
            .object_store::<IssuesInitialSyncStatus>()?
            .get(id)
            .await?;
        if let Some(initial_sync_status) = initial_sync_status {
            match initial_sync_status.status {
                InitialSyncStatusEnum::Full => return Ok(()),
                InitialSyncStatusEnum::Partial => {
                    page = (txn
                        .object_store::<Issue>()?
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
            .ro();
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
            let issues = github_api::apis::issues_api::issues_slash_list_for_repo(
                &conf,
                &owner_name,
                &repo_name,
                None,
                None,
                None,
                None,
                None,
                None,
                "created".into(),
                "asc".into(),
                None,
                MAX_PER_PAGE.into(),
                page.into(),
            )
            .await?;
            let last_fetched_num = issues.len();
            let changes = Changes::try_try_from_iter(
                issues.into_iter().map(|r| from_issue(r, id).map(|r| r.1)),
            )??;

            let txn = Changes::txn(&self.db)
                .with_store::<IssuesInitialSyncStatus>()
                .rw();
            self.merge_and_upsert_changes(&txn, changes).await?;
            txn.object_store::<IssuesInitialSyncStatus>()?
                .put(&IssuesInitialSyncStatus {
                    status: InitialSyncStatusEnum::Partial,
                    id: *id,
                })
                .await?;

            page += 1;
            if last_fetched_num < MAX_PER_PAGE as usize {
                break;
            }
        }

        let txn = self.db.txn().with_store::<IssuesInitialSyncStatus>().rw();
        txn.object_store::<IssuesInitialSyncStatus>()?
            .put(&IssuesInitialSyncStatus {
                status: InitialSyncStatusEnum::Full,
                id: *id,
            })
            .await?;
        Ok(())
    }
}

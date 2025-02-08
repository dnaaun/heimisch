use futures::future::try_join_all;
use github_api::github_api_trait::GithubApiTrait;

use crate::{
    avail::MergeError, sync_engine::websocket_updates::typed_transport::TypedTransportTrait,
    types::installation::InstallationId,
};

use super::super::{
    changes::{AddChanges, Changes},
    conversions::ToDb,
    SyncEngine, SyncResult, MAX_PER_PAGE,
};

impl<W: TypedTransportTrait, GithubApi: GithubApiTrait> SyncEngine<W, GithubApi> {
    pub async fn fetch_repositorys_for_installation_id(
        &self,
        id: &InstallationId,
    ) -> SyncResult<(), W> {
        let conf = self.get_api_conf(id).await?;

        let mut repos = vec![];
        let mut page = 1;
        loop {
            let repos_in_page =
                GithubApi::apps_slash_list_repos_accessible_to_installation(
                    &conf,
                    Some(MAX_PER_PAGE),
                    Some(page),
                )
                .await?
                .repositories;
            let last_fetched_num = repos_in_page.len();
            repos.extend(repos_in_page);
            page += 1;
            if last_fetched_num < MAX_PER_PAGE as usize {
                break;
            }
        }

        let changes = try_join_all(
            repos
                .into_iter()
                .map(|r| r.try_to_db_type_and_other_changes(*id)),
        )
        .await?
        .into_iter()
        .try_fold(Changes::default(), |mut acc, (repo, other_changes)| {
            acc.add(repo)?;
            acc.add(other_changes)?;
            Ok::<_, MergeError>(acc)
        })?;

        let txn = Changes::txn(&self.db).read_write().build();
        self.persist_changes(&txn, changes).await?;

        Ok(())
    }
}

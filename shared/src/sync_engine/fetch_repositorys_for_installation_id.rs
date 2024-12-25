use crate::types::installation::InstallationId;

use super::{
    changes::Changes, conversions::from_repository::from_repository,
    typed_transport::TypedTransportTrait, SyncEngine, SyncResult, MAX_PER_PAGE,
};

impl<W: TypedTransportTrait> SyncEngine<W> {
    pub async fn fetch_repositorys_for_installation_id(
        &self,
        id: &InstallationId,
    ) -> SyncResult<(), W> {
        let conf = self.get_api_conf(id).await?;

        let mut repos = vec![];
        let mut page = 1;
        loop {
            let repos_in_page =
                github_api::apis::apps_api::apps_slash_list_repos_accessible_to_installation(
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

        let changes = repos
            .into_iter()
            .map(|r| from_repository(r, *id).map(|r| r.1))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .try_fold(Changes::default(), |acc, new| acc.with_added(new))?;

        let txn = Changes::txn(&self.db).read_write().build();
        self.merge_and_upsert_changes(&txn, changes).await?;

        Ok(())
    }
}

use crate::avail::Avail;
use crate::sync_engine::changes::AddChanges;
use crate::sync_engine::conversions::to_db::*;
use crate::{
    avail::MergeError,
    sync_engine::{changes::Changes, conversions::ToDb},
    types::github_app::GithubApp,
};

impl ToDb for github_api::models::App10 {
    type DbType = Option<GithubApp>;

    type Error = MergeError;

    type OtherChanges = Changes;

    type Args = ();

    fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::App10 {
            created_at,
            description,
            events,
            external_url,
            html_url,
            id,
            name,
            node_id,
            owner,
            permissions,
            slug,
            updated_at,
        } = self;

        let db_owner = owner.map(|o| (*o).to_db_type(()));

        let db_app = id.map(|id| crate::types::github_app::GithubApp {
            client_id: Avail::No,
            client_secret: Avail::No,
            created_at: Avail::from_option(created_at),
            description: description.into(),
            events: Avail::from_option(events),
            external_url: Avail::from_option(external_url),
            html_url: html_url.into(),
            id: (id as i64).into(),
            installations_count: Avail::No,
            name: name.into(),
            node_id: node_id.into(),
            owner_id: Avail::from_option(db_owner.as_ref().map(|o| o.id)),
            pem: Avail::No,
            permissions: Avail::from_option(permissions.map(|p| *p)),
            slug: slug.into(),
            updated_at: Avail::from_option(updated_at),
            webhook_secret: Avail::No,
        });

        let mut changes = Changes::default();
        changes.add(db_owner)?;

        Ok((db_app, changes))
    }
}

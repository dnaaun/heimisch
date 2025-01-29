use futures::future::OptionFuture;

use crate::avail::Avail;
use crate::{
    avail::MergeError,
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::{InfallibleToDbNoOtherChanges, ToDb},
    },
};

impl ToDb for github_api::models::NullableIntegration {
    type DbType = crate::types::github_app::GithubApp;

    type Error = MergeError;

    type OtherChanges = Changes;

    type Args = ();

    async fn try_to_db_type_and_other_changes(
        self,
        _args: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::NullableIntegration {
            id,
            slug,
            node_id,
            client_id,
            owner,
            name,
            description,
            external_url,
            html_url,
            created_at,
            updated_at,
            permissions,
            events,
            installations_count,
            client_secret,
            webhook_secret,
            pem,
        } = self;

        let db_owner = OptionFuture::from(owner.map(|i| i.to_db_type(()))).await;

        let db_app = crate::types::github_app::GithubApp {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            created_at: created_at.into(),
            description: description.into(),
            events: events.into(),
            external_url: external_url.into(),
            html_url: html_url.into(),
            id: (id as i64).into(),
            installations_count: Avail::from_option(installations_count.map(i64::from)),
            name: name.into(),
            node_id: node_id.into(),
            owner_id: Avail::from_option(db_owner.as_ref().map(|o| o.id)),
            pem: pem.into(),
            permissions: Avail::Yes(permissions.into()),
            slug: slug.into(),
            updated_at: updated_at.into(),
            webhook_secret: Avail::from_option(webhook_secret),
        };

        let mut changes = Changes::default();
        changes.add(db_owner)?;

        Ok((db_app, changes))
    }
}

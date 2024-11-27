use tracing::warn;

use crate::sync_engine::changes::AddChanges;
use crate::{
    avail::{Avail, MergeError},
    sync_engine::changes::Changes,
    types::github_app::GithubAppId,
};

use super::from_nullable_simple_user::from_nullable_simple_user;
use super::from_user2::from_user2;

pub fn from_nullable_integration(
    api_integration: github_api::models::NullableIntegration,
) -> Result<(GithubAppId, Changes), MergeError> {
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
    } = api_integration;

    let db_owner = owner.map(|i| from_nullable_simple_user(*i));

    let db_app = crate::types::github_app::GithubApp {
        client_id: client_id.into(),
        client_secret: client_secret.into(),
        created_at: created_at.into(),
        description: description.into(),
        events: events.into(),
        external_url: external_url.into(),
        html_url: html_url.into(),
        id: (id as i64).into(),
        installations_count: Avail::No,
        name: name.into(),
        node_id: node_id.into(),
        owner_id: Avail::from_option(db_owner.as_ref().map(|o| o.id)),
        pem: Avail::No,
        permissions: Avail::Yes(permissions.into()),
        slug: slug.into(),
        updated_at: updated_at.into(),
        webhook_secret: Avail::from_option(webhook_secret),
    };
    let id = db_app.id.clone();

    let mut changes = Changes::default();
    changes.add(db_app)?.add(db_owner)?;

    Ok((id, changes))
}

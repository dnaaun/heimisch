use crate::sync_engine::changes::AddChanges;
use crate::{
    avail::{Avail, MergeError},
    sync_engine::changes::Changes,
    types::github_app::GithubAppId,
};

use super::from_user2::from_user2;

/// Will return a None github app if the id is None.
pub fn from_app10(
    api_app10: github_api::models::App10,
) -> Result<(Option<GithubAppId>, Changes), MergeError> {
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
    } = api_app10;

    let db_owner = owner.map(|i| from_user2(*i));

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
    let id = db_app.as_ref().map(|d| d.id);

    let mut changes = Changes::default();
    changes.add(db_app)?.add(db_owner)?;

    Ok((id, changes))
}

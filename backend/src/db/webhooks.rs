use crate::error::Result;
use diesel::prelude::*;
use std::{ops::Deref, time::SystemTime};

use super::schema::webhooks::*;
use deadpool_diesel::postgres::Pool;
use github_webhook_body::WebhookBody;
use shared::types::installation::InstallationId;

pub async fn upsert_webhook(
    pool: impl AsRef<Pool>,
    id_arg: i64,
    installation_id_arg: InstallationId,
    webhook_content_arg: WebhookBody,
) -> Result<()> {
    let webhook_content_arg = serde_json::to_value(webhook_content_arg).expect("");
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        diesel::insert_into(table)
            .values((
                id.eq(id_arg),
                installation_id.eq(installation_id_arg.deref()),
                webhook_content.eq(&webhook_content_arg),
                created_at.eq(SystemTime::now()),
            ))
            .on_conflict(id)
            .do_update()
            .set((
                installation_id.eq(installation_id_arg.deref()),
                webhook_content.eq(&webhook_content_arg),
            ))
            .execute(conn)
    })
    .await??;

    Ok(())
}

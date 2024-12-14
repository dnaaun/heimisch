use crate::error::{DbIntegrityError, Result};
use diesel::prelude::*;
use jiff::{SignedDuration, Timestamp};
use std::ops::Deref;

use super::schema::webhooks::*;
use super::schema::*;
use deadpool_diesel::postgres::Pool;
use github_webhook_body::WebhookBody;
use shared::{
    endpoints::defns::api::websocket_updates::Webhook,
    types::{installation::InstallationId, user::UserId},
};

pub async fn upsert_webhook(
    pool: impl AsRef<Pool>,
    id_arg: i64,
    installation_id_arg: InstallationId,
    webhook_content_arg: &WebhookBody,
) -> Result<Timestamp> {
    let webhook_content_arg = serde_json::to_value(webhook_content_arg).expect("");
    let conn = pool.as_ref().get().await?;
    let created_at_val = conn
        .interact(move |conn| {
            diesel::insert_into(table)
                .values((
                    id.eq(id_arg),
                    installation_id.eq(installation_id_arg.deref()),
                    webhook_content.eq(&webhook_content_arg),
                    created_at.eq(SystemTime::now()),
                ))
                .returning(created_at)
                .on_conflict(id)
                .do_update()
                .set((
                    installation_id.eq(installation_id_arg.deref()),
                    webhook_content.eq(&webhook_content_arg),
                ))
                .load::<SystemTime>(conn)
        })
        .await??;

    let mut created_at_val_iter = created_at_val.into_iter();
    let created_at_val = created_at_val_iter.next();

    match (created_at_val, created_at_val_iter.next()) {
        (Some(created_at_val), None) => Ok(convert_system_time_to_jiff(created_at_val)),
        _ => panic!("on_conflict(id) should ensure exactly one row is inserted/updated"),
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn convert_system_time_to_jiff(system_time: SystemTime) -> jiff::Timestamp {
    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
    jiff::Timestamp::from_jiff_duration(SignedDuration::from_nanos(
        duration_since_epoch.as_nanos().try_into().unwrap(),
    ))
    .unwrap()
}

// pub fn convert_jiff_to_system_time(timestamp: jiff::Timestamp) -> SystemTime {
//     // Convert jiff::Timestamp to a duration since the Unix epoch
//     let signed_duration = timestamp.as_jiff_duration();
//
//     // Convert the SignedDuration to nanos
//     let nanos_since_epoch = signed_duration.as_nanos().try_into().unwrap();
//
//     // Create a Duration from the nanos
//     let duration = std::time::Duration::from_nanos(nanos_since_epoch);
//
//     UNIX_EPOCH + duration
// }

// As I do not have access to jiff crate documentation, you will need to replace the placeholder lines with the actual jiff conversion calls.

pub async fn get_webhooks_for_user(
    pool: impl AsRef<Pool>,
    user_id: UserId,
    // since: Timestamp,
) -> Result<Vec<Webhook>> {
    let conn = pool.as_ref().get().await?;
    let result: Vec<(i64, SystemTime, serde_json::Value)> = conn
        .interact(move |conn| {
            let query = table
                .inner_join(
                    installations::table.inner_join(
                        login_users::table
                            .on(login_users::github_user_id.eq(installations::github_user_id)),
                    ),
                )
                .select((id, created_at, webhook_content))
                .filter(login_users::github_user_id.eq_all(user_id))
                // .filter(created_at.ge(convert_jiff_to_system_time(since))) 
                ;

            query.load::<(i64, SystemTime, serde_json::Value)>(conn)
        })
        .await??;

    result
        .into_iter()
        .map(|(id_value, created_at_value, webhook_content_value)| {
            let body = serde_json::from_value::<WebhookBody>(webhook_content_value.clone())
                .map_err(
                    move |err| DbIntegrityError::WebhookWebhookContentIsNotValid {
                        webhook_id: id_value,
                        webhook_content: webhook_content_value,
                        error: err,
                    },
                )?;
            Ok(Webhook {
                body,
                created_at: convert_system_time_to_jiff(created_at_value),
            })
        })
        .collect()
}

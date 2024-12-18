pub mod applying_error;

use applying_error::{ApplyingError, ApplyingResult};
use futures::{pin_mut, StreamExt};

use crate::{
    endpoints::defns::api::websocket_updates::{ServerMsg, WEBSOCKET_UPDATES_ENDPOINT},
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::ToDb,
    },
};

use super::{error::SyncErrorSrc, SyncEngine, SyncResult, WSClient};

impl<W> SyncEngine<W>
where
    W: WSClient,
{
    pub async fn recv_websocket_updates(&self) -> SyncResult<(), W::Error> {
        loop {
            let (_, recver) = W::establish(WEBSOCKET_UPDATES_ENDPOINT)
                .await
                .map_err(SyncErrorSrc::WebSocket)?;
            pin_mut!(recver);
            loop {
                let fut = recver.next();
                pin_mut!(fut);
                match fut.await {
                    Some(value) => match value {
                        Ok(server_msg) => match self.apply_update_to_db(&server_msg).await {
                            Ok(_) => tracing::info!(
                                "Successfully applied webhook update: {server_msg:?}"
                            ),
                            Err(err) => {
                                let serialized = match serde_json::to_string_pretty(&server_msg) {
                                    Ok(s) => s,
                                    Err(e) => format!("error serializing update to json {e:?}"),
                                };

                                match err {
                                    ApplyingError::NotImplemented => tracing::info!(
                                    "LOCAL DB UPDATES FOR WEBHOOK NOT IMPLEMENTED: {serialized}"
                                ),
                                    ApplyingError::Sync(sync_error) => {
                                        tracing::error!(
                                            "Error applying update:
{}
{:?}",
                                            serialized,
                                            sync_error
                                        )
                                    }
                                }
                            }
                        },
                        Err(err) => {
                            tracing::error!("{:?}", err)
                        }
                    },
                    None => break,
                }
            }
        }
    }

    async fn apply_update_to_db(&self, _server_msg: &ServerMsg) -> ApplyingResult<(), W::Error> {
        use github_webhook_body::*;
        let changes = match _server_msg.body.clone() {
            WebhookBody::Issues(issues) => {
                let (issue, mut other_changes) = issues.try_to_db_type_and_other_changes(())?;
                other_changes.add(issue)?;
                other_changes
            }
            WebhookBody::IssueComment(issue_comment) => {
                let (issue, mut other_changes) =
                    issue_comment.try_to_db_type_and_other_changes(())?;
                other_changes.add(issue)?;
                other_changes
            }
            _ => return Err(ApplyingError::NotImplemented),
        };

        let txn = Changes::txn(&self.db).read_write().build();
        self.merge_and_upsert_changes(txn, changes).await?;
        Ok(())
    }
}

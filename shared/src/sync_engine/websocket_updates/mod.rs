pub mod applying_error;
pub mod binary_transport;

pub mod transport;

use applying_error::{ApplyingError, ApplyingResult};
use futures::{pin_mut, StreamExt};
use transport::TransportTrait;
use typed_db::RawDbTrait;

use crate::{
    backend_api_trait::BackendApiTrait,
    endpoints::defns::api::websocket_updates::{
        ServerMsg, WebsocketUpdatesQueryParams, WEBSOCKET_UPDATES_ENDPOINT,
    },
    retry::try_n_times,
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::ToDb,
    },
    types::last_webhook_update_at::{LastWebhookUpdateAt, LastWebhookUpdateAtId},
};

use super::{error::SyncErrorSrc, SyncEngine, SyncResult};

impl<RawDb: RawDbTrait, BackendApi: BackendApiTrait, Transport: TransportTrait, GithubApi>
    SyncEngine<RawDb, BackendApi, Transport, GithubApi>
where
    Transport: TransportTrait,
{
    pub async fn recv_websocket_updates(&self) -> SyncResult<(), Transport> {
        let mut url = self
            .backend_api
            .get_domain()
            .join(WEBSOCKET_UPDATES_ENDPOINT)
            .expect("");

        let last_webhook_update_at = self
            .db
            .txn()
            .with_table::<LastWebhookUpdateAt>()
            .build()
            .table::<LastWebhookUpdateAt>()?
            .get(&LastWebhookUpdateAtId::Singleton)
            .await?;
        url.set_query(Some(
            &serde_urlencoded::to_string(&WebsocketUpdatesQueryParams {
                return_backlog_after: last_webhook_update_at.map(|l| l.at),
            })
            .expect(""),
        ));
        let websocket_conn = try_n_times(|| (self.make_transport)(url.clone()), 3)
            .await
            .map_err(|e| SyncErrorSrc::Transport(e))?;
        pin_mut!(websocket_conn);
        loop {
            let fut = websocket_conn.next();
            pin_mut!(fut);
            tracing::info!("Waiting for websocket updates");
            match fut.await {
                Some(value) => {
                    tracing::info!("Received websocket update: {:?}", value);
                    match value {
                        Ok(server_msg) => match self.apply_update_to_db(&server_msg).await {
                            Ok(_) => {
                                tracing::info!(
                                    "Successfully applied webhook update: {server_msg:?}"
                                )
                            }
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
                            tracing::error!("Error receiving websocket update: {:?}", err);
                        }
                    }
                }
                None => return Ok(()),
            }
        }
    }

    async fn apply_update_to_db(&self, _server_msg: &ServerMsg) -> ApplyingResult<(), Transport> {
        use github_webhook_body::*;
        let changes = match _server_msg.body.clone() {
            WebhookBody::Issues(issues) => {
                let (issue, mut other_changes) =
                    issues.try_to_db_type_and_other_changes(()).await?;
                other_changes.add(issue)?;
                other_changes
            }
            WebhookBody::IssueComment(issue_comment) => {
                let (issue, mut other_changes) =
                    issue_comment.try_to_db_type_and_other_changes(()).await?;
                other_changes.add(issue)?;
                other_changes
            }
            _ => return Err(ApplyingError::NotImplemented),
        };

        let txn = Changes::txn(&self.db)
            .with_store::<LastWebhookUpdateAt>()
            .read_write()
            .build();
        self.persist_changes(&txn, changes).await?;
        txn.table::<LastWebhookUpdateAt>()?
            .put(&LastWebhookUpdateAt {
                at: jiff::Timestamp::now(),
                id: Default::default(),
            })
            .await?;
        drop(txn);
        Ok(())
    }
}

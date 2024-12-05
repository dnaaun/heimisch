use github_webhook_body::WebhookBody;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

pub const WEBSOCKET_UPDATES_ENDPOINT: &str = "/api/websocket_updates";

#[derive(Serialize, Deserialize, Clone)]
pub struct Webhook {
    pub body: WebhookBody,
    pub created_at: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMsg {
    InitialBacklog(Vec<Webhook>),
    One(Webhook),
}

#[derive(Serialize, Deserialize)]
pub enum ClientMsg {
}

#[derive(Serialize, Deserialize)]
pub struct WebsocketUpdatesPayload {
    pub updates_since: Timestamp,
}

use serde::{Deserialize, Serialize};

pub const WEBSOCKET_UPDATES_URL: &'static str = "/api/websocket_updates";

#[derive(Serialize, Deserialize)]
pub enum ServerMsg {
    Hi,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMsg {
    GetUpdatesSince(jiff::Timestamp),
}

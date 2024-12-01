use axum::{routing::get, Router};
use axum_typed_websockets::{WebSocket, WebSocketUpgrade};
use shared::endpoints::defns::api::websocket_updates::{
    ClientMsg, ServerMsg, WEBSOCKET_UPDATES_URL,
};

use crate::app_state::AppState;

pub async fn _api_websocket_updates(router: Router<AppState>) -> Router<AppState> {
    router.route(
        WEBSOCKET_UPDATES_URL,
        get(|ws: WebSocketUpgrade<ServerMsg, ClientMsg>| async {
            ws.on_upgrade(_handle_websocket_updates)
        }),
    )
}

async fn _handle_websocket_updates(mut _socket: WebSocket<ServerMsg, ClientMsg>) {}

// mod updates_state_machine {
//     use super::*;
//     use axum_typed_websockets::{Message, WebSocket};
//     use tokio::spawn;
//
//     pub struct NotInitialized {
//         socket: WebSocket<ServerMsg, ClientMsg>,
//     }
//
//     impl NotInitialized {
//         pub fn new(mut socket: WebSocket<ServerMsg, ClientMsg>) {
//             let not_initialized = Self { socket };
//
//             spawn(async {
//                 loop {
//                     let msg = not_initialized.socket.recv().await;
//                     if let Some(Ok(msg)) = msg {
//                         match msg {
//                             Message::Item(client_msg) => todo!(),
//                             Message::Ping(vec) => {
//                                 match not_initialized.socket.send(Message::Pong(vec)).await {
//                                     Ok(_) => (),
//                                     Err(_) => break,
//                                 }
//                             }
//                             Message::Close(close_frame) => break,
//                             Message::Pong(vec) => (),
//                         }
//                     }
//                 }
//             })
//         }
//     }
//
//     pub struct ProvideUpdatesSince {
//         since: jiff::Timestamp,
//         socket: WebSocket<ServerMsg, ClientMsg>,
//     }
// }

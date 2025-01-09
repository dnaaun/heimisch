use shared::{
    endpoints::defns::api::websocket_updates::WebsocketUpdatesQueryParams, utils::LogErr,
};
use tokio::sync::mpsc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use futures::{SinkExt, StreamExt};
use shared::{
    endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg, WEBSOCKET_UPDATES_ENDPOINT},
    types::user::UserId,
};

use crate::{
    app_state::AppState, auth_backend::AuthBackend, axum_helpers::extractors::AuthenticatedUser,
    db::get_webhooks_for_user_asc, websocket_updates_bucket::DEFAULT_CAPACITY,
};

pub fn api_websocket_updates(router: Router<AppState>) -> Router<AppState> {
    router.route(WEBSOCKET_UPDATES_ENDPOINT, get(inner))
}

async fn inner(
    auth_user: AuthenticatedUser<AuthBackend>,
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
    State(app_state): State<AppState>,
    Query(query): Query<WebsocketUpdatesQueryParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_websocket_updates(app_state, auth_user.github_user_id, query, socket)
    })
}

async fn handle_websocket_updates(
    app_state: AppState,
    user_id: UserId,
    WebsocketUpdatesQueryParams {
        return_backlog_after,
    }: WebsocketUpdatesQueryParams,
    socket: WebSocket<ServerMsg, ClientMsg>,
) {
    let backlog = match return_backlog_after {
        Some(return_backlog_after) => {
            match get_webhooks_for_user_asc(&app_state, user_id, return_backlog_after)
                .await
                .log_err()
            {
                Ok(b) => b,
                Err(_) => return,
            }
        }
        None => vec![],
    };
    // Note that the backlog should be at most that of a week, so I hope this is ok.
    let capacity = DEFAULT_CAPACITY.max(backlog.len());
    let (tx, mut rx) = mpsc::channel(capacity);
    for server_msg in backlog {
        if tx.send(server_msg).await.log_err().is_err() {
            let _ = socket.close().await.log_err();
            return;
        }
    }

    tokio::spawn(async move {
        let (mut socket_writer, mut socket_reader) = socket.split();
        loop {
            tokio::select! {
                client_msg = socket_reader.next() => {
                    match client_msg {
                        Some(msg) => match msg.log_err() {
                            Ok(msg) => match msg {
                                Message::Ping(vec) => {
                                    if socket_writer.send(Message::Pong(vec)).await.log_err().is_err() {
                                        break;
                                    };
                                }
                                Message::Item(_) | Message::Pong(_) => (),
                                Message::Close(_) => break,
                            },
                            Err(_) => {
                                break;
                            }
                        },
                        None => break,
                    }
                }
                server_msg = rx.recv() => {
                    let server_msg = match server_msg {
                        Some(s) => s,
                        None => break
                    };
                    if socket_writer.send(Message::Item(server_msg)).await.log_err().is_err() {
                        break;
                    }
                }
            };
        }
    });

    let mut subscription = app_state.websocket_updates_bucket.subscribe(user_id);

    loop {
        let server_msg = match subscription.recv().await.log_err() {
            Ok(u) => u,
            Err(_) => {
                return;
            }
        };

        if tx.send(server_msg).await.log_err().is_err() {
            return;
        }
    }
}

// NOTE: Umm, write tests.

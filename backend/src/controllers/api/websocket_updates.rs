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
    endpoints::defns::api::websocket_updates::{
        ClientMsg, ServerMsg, WebsocketUpdatesQueryParams, WEBSOCKET_UPDATES_ENDPOINT,
    },
    types::user::UserId,
};

use crate::{
    app_state::AppState, auth_backend::AuthBackend, axum_helpers::extractors::AuthenticatedUser,
    db::get_webhooks_for_user_since, error::LogErr, websocket_updates_bucket::CAPACITY,
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
    println!("got request to create websocket connection");
    ws.on_upgrade(move |socket| {
        handle_websocket_updates(app_state, auth_user.github_user_id, query, socket)
    })
}

async fn handle_websocket_updates(
    app_state: AppState,
    user_id: UserId,
    WebsocketUpdatesQueryParams { updates_since }: WebsocketUpdatesQueryParams,
    socket: WebSocket<ServerMsg, ClientMsg>,
) {
    println!("handling websocket updates");
    let (tx, mut rx) = mpsc::channel(CAPACITY);
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
                    if let Err(_) = socket_writer.send(Message::Item(server_msg)).await.log_err() {
                        break;
                    }
                }
            };
        }
    });

    let initial_backlog = match get_webhooks_for_user_since(&app_state, user_id, updates_since)
        .await
        .log_err()
    {
        Ok(initial_backlog) => initial_backlog,
        Err(_) => {
            return;
        }
    };

    if !initial_backlog.is_empty() {
        if tx
            .send(ServerMsg::InitialBacklog(initial_backlog))
            .await
            .log_err()
            .is_err()
        {
            return;
        }
    }

    let mut subscription = app_state.websocket_updates_bucket.subscribe(user_id);

    loop {
        println!("About to wait for updates to send on websocket.");
        let update = match subscription.recv().await.log_err() {
            Ok(u) => u,
            Err(_) => {
                return;
            }
        };

        if let Err(_) = tx.send(ServerMsg::One(update)).await.log_err() {
            return;
        }
    }
}

// TODO: Umm, write tests.
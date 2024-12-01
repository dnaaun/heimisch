use crate::session_and_auth::AuthBackend;
use std::future::Future;

use axum::{
    extract,
    response::IntoResponse,
    routing::{get, post},
    Json,
};
use axum_login::AuthSession;
use http::StatusCode;

pub trait HookupEndpoint<State, Endpoint, Error, Fut, Func> {
    fn hookup(self, endpoint: Endpoint, func: Func) -> Self;
}

impl<State, Endpoint, Error, Fut, Func> HookupEndpoint<State, Endpoint, Error, Fut, Func>
    for axum::Router<State>
where
    Endpoint: shared::endpoints::endpoint::Endpoint,
    Endpoint::JsonPayload: Send + 'static,
    Error: IntoResponse,
    Fut: Future<Output = Result<(StatusCode, Endpoint::JsonResponse), Error>> + Send,
    Func: FnOnce(AuthSession<AuthBackend>, State, Endpoint::JsonPayload) -> Fut
        + Send
        + 'static
        + Clone,
    State: Clone + Send + Sync + 'static,
{
    fn hookup(self, _endpoint: Endpoint, func: Func) -> Self {
        let method_body = |auth_session: AuthSession<AuthBackend>,
                           extract::State(state): extract::State<_>,
                           Json(json): Json<_>| async {
            func(auth_session, state, json)
                .await
                .map(|(status_code, response)| (status_code, Json(response)))
        };
        match Endpoint::METHOD {
            shared::endpoints::endpoint::Method::Post => {
                self.route(Endpoint::PATH, post(method_body))
            }
            shared::endpoints::endpoint::Method::Get => {
                self.route(Endpoint::PATH, get(method_body))
            }
        }
    }
}

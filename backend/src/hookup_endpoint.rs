use std::future::Future;

use axum::{
    extract,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_login::AuthSession;
use http::StatusCode;

use crate::{auth_backend::AuthBackend, axum_helpers::extractors::AuthenticatedUser};

pub fn hookup<Endpoint, State, Error, Fut, Func>(
    _: Endpoint,
    router: Router<State>,
    func: Func,
) -> Router<State>
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
    let method_body = |auth_session: AuthSession<AuthBackend>,
                       extract::State(state): extract::State<_>,
                       Json(json): Json<_>| async {
        func(auth_session, state, json)
            .await
            .map(|(status_code, response)| (status_code, Json(response)))
    };
    match Endpoint::METHOD {
        shared::endpoints::endpoint::Method::Post => {
            router.route(Endpoint::PATH, post(method_body))
        }
        shared::endpoints::endpoint::Method::Get => router.route(Endpoint::PATH, get(method_body)),
    }
}

pub fn hookup_authenticated<Endpoint, State, Error, Fut, Func>(
    _: Endpoint,
    router: Router<State>,
    func: Func,
) -> Router<State>
where
    Endpoint: shared::endpoints::endpoint::Endpoint,
    Endpoint::JsonPayload: Send + 'static,
    Error: IntoResponse,
    Fut: Future<Output = Result<(StatusCode, Endpoint::JsonResponse), Error>> + Send,
    Func: FnOnce(AuthenticatedUser<AuthBackend>, State, Endpoint::JsonPayload) -> Fut
        + Send
        + 'static
        + Clone,
    State: Clone + Send + Sync + 'static,
{
    let method_body = |authenticated_user: AuthenticatedUser<AuthBackend>,
                       extract::State(state): extract::State<_>,
                       Json(json): Json<_>| async {
        func(authenticated_user, state, json)
            .await
            .map(|(status_code, response)| (status_code, Json(response)))
    };
    match Endpoint::METHOD {
        shared::endpoints::endpoint::Method::Post => {
            router.route(Endpoint::PATH, post(method_body))
        }
        shared::endpoints::endpoint::Method::Get => router.route(Endpoint::PATH, get(method_body)),
    }
}

use axum::response::IntoResponse;
use std::{future::Future, pin::Pin};
use tower::util::ServiceExt;

use axum::{
    body::Body,
    extract::{FromRef, State},
    response::Response,
};
use http::{HeaderMap, HeaderValue, Request, StatusCode, Uri};
use leptos::{config::LeptosOptions, IntoView};
use tower_http::services::ServeDir;

async fn get_static_file(
    uri: Uri,
    root: &str,
    headers: &HeaderMap<HeaderValue>,
) -> Result<Response<Body>, (StatusCode, String)> {
    use axum::http::header::ACCEPT_ENCODING;

    let req = Request::builder().uri(uri);

    let req = match headers.get(ACCEPT_ENCODING) {
        Some(value) => req.header(ACCEPT_ENCODING, value),
        None => req,
    };

    let req = req.body(Body::empty()).unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root)
        .precompressed_gzip()
        .precompressed_br()
        .oneshot(req)
        .await
    {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
/// A reasonable handler for serving static files (like JS/WASM/CSS) and 404 errors.
///
/// This is provided as a convenience, but is a fairly simple function. If you need to adapt it,
/// simply reuse the source code of this function in your own application.
pub fn file_and_error_handler<S, IV>() -> impl Fn(
    Uri,
    State<S>,
    Request<Body>,
) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + 'static>>
       + Clone
       + Send
       + 'static
where
    IV: IntoView + 'static,
    S: Send + 'static,
    LeptosOptions: FromRef<S>,
{
    move |uri: Uri, State(options): State<S>, req: Request<Body>| {
        Box::pin(async move {
            let options = LeptosOptions::from_ref(&options);
            let res = get_static_file(uri, &options.site_root, req.headers());
            let res = res.await.unwrap();

            res.into_response()
        })
    }
}

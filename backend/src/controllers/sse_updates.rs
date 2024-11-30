// use std::time::Duration;

// use axum::{
//     response::{IntoResponse, Sse},
//     Router,
// };

// use crate::app_state::AppState;

// pub async fn api_sse_updates(router: Router<AppState>) -> Router<AppState> {}

// async fn sse_updates_handler() -> impl IntoResponse {
//     // A `Stream` that repeats an event every second
//     //
//     // You can also create streams from tokio channels using the wrappers in
//     // https://docs.rs/tokio-stream
//     let stream = stream::repeat_with(|| Event::default().data("hi!"))
//         .map(Ok)
//         .throttle(Duration::from_secs(1));
//
//     let sse = Sse::new(stream).keep_alive(
//         axum::response::sse::KeepAlive::new()
//             .interval(Duration::from_secs(10))
//             .text("keep-alive-text"),
//     );
//
//     sse
// }

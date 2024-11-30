use codee::string::JsonSerdeCodec;
use leptos_use::{use_websocket, UseWebSocketReturn};
use shared::endpoints::defns::api::websocket_updates::{
    ClientMsg, ServerMsg, WEBSOCKET_UPDATES_URL,
};

pub fn use_websocket_updates() -> UseWebSocketReturn<
    ClientMsg,
    ServerMsg,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn(&ClientMsg) + Clone + Send + Sync,
> {
    use_websocket::<ClientMsg, ServerMsg, JsonSerdeCodec>(WEBSOCKET_UPDATES_URL)
}

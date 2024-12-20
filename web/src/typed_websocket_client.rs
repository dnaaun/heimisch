use std::{
    marker::PhantomData,
    task::{ready, Poll},
};

use codee::{Decoder, Encoder};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use gloo_net::websocket::{self, futures::WebSocket, Message};
use gloo_utils::errors::JsError;
use pin_project::pin_project;
use url::Url;

trait BinaryEncoder<T>: Encoder<T, Encoded = Vec<u8>> {
    // type Error;
}
impl<Codec, T> BinaryEncoder<T> for Codec
where
    Codec: Encoder<T, Encoded = Vec<u8>>,
{
    // type Error = Codec::Error;
}

trait BinaryDecoder<T>: Decoder<T, Encoded = [u8]> {
    // type Error;
}

impl<Codec, T> BinaryDecoder<T> for Codec
where
    Codec: Decoder<T, Encoded = [u8]>,
{
    // type Error = Codec::Error;
}

pub struct TypedWebsocketClient;

#[derive(Debug)]
pub enum Error<E, D> {
    Encode(E),
    Decode(D),
    GlooJs(JsError),
    GlooWebsocket(gloo_net::websocket::WebSocketError),
    Send(futures::channel::mpsc::SendError),
    TryRecv(futures::channel::mpsc::TryRecvError),
}

impl<E, D> From<futures::channel::mpsc::SendError> for Error<E, D> {
    fn from(value: futures::channel::mpsc::SendError) -> Self {
        Error::Send(value)
    }
}

impl<E, D> From<JsError> for Error<E, D> {
    fn from(value: JsError) -> Self {
        Error::GlooJs(value)
    }
}

impl<E, D> From<gloo_net::websocket::WebSocketError> for Error<E, D> {
    fn from(value: gloo_net::websocket::WebSocketError) -> Self {
        Error::GlooWebsocket(value)
    }
}

impl<E, D> From<futures::channel::mpsc::TryRecvError> for Error<E, D> {
    fn from(value: futures::channel::mpsc::TryRecvError) -> Self {
        Error::TryRecv(value)
    }
}

fn from_gloo<T, Codec: BinaryDecoder<T>>(gloo_msg: &Message) -> Result<T, Codec::Error> {
    Codec::decode(match gloo_msg {
        websocket::Message::Text(t) => t.as_bytes(),
        websocket::Message::Bytes(vec) => vec,
    })
}

fn to_gloo<T, Codec: BinaryEncoder<T>>(item: &T) -> Result<Message, Codec::Error> {
    Ok(Message::Bytes(Codec::encode(item)?))
}

#[pin_project]
pub struct Sender<Codec, ClientType, ServerType> {
    #[pin]
    sink: SplitSink<WebSocket, gloo_net::websocket::Message>,
    _types: PhantomData<(Codec, ClientType, ServerType)>,
}

impl<Codec, ClientType, ServerType> futures::Sink<ClientType>
    for Sender<Codec, ClientType, ServerType>
where
    Codec: BinaryDecoder<ServerType> + BinaryEncoder<ClientType>,
{
    type Error =
        Error<<Codec as Encoder<ClientType>>::Error, <Codec as Decoder<ServerType>>::Error>;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(Ok(ready!(this.sink.poll_ready(cx))?))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: ClientType) -> Result<(), Self::Error> {
        let this = self.project();
        Ok(this
            .sink
            .start_send(to_gloo::<ClientType, Codec>(&item).map_err(Error::Encode)?)?)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.sink.poll_flush(cx)).map_err(Error::from))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.sink.poll_close(cx)).map_err(Error::from))
    }
}

#[pin_project]
pub struct Receiver<Codec, ClientType, ServerType> {
    #[pin]
    stream: SplitStream<WebSocket>,
    _types: PhantomData<(Codec, ClientType, ServerType)>,
}

impl<Codec, ClientType, ServerType> futures::Stream for Receiver<Codec, ClientType, ServerType>
where
    Codec: BinaryDecoder<ServerType> + BinaryEncoder<ClientType>,
{
    type Item = Result<
        ServerType,
        Error<<Codec as Encoder<ClientType>>::Error, <Codec as Decoder<ServerType>>::Error>,
    >;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let item = ready!(this.stream.poll_next(cx));
        Poll::Ready(item.map(|item| from_gloo::<ServerType, Codec>(&item?).map_err(Error::Decode)))
    }
}

impl<ClientType, ServerType, Codec>
    shared::sync_engine::TypedWebsocketClient<ClientType, ServerType, Codec>
    for TypedWebsocketClient
where
    Codec: BinaryEncoder<ClientType> + BinaryDecoder<ServerType>,
    <Codec as codee::Encoder<ClientType>>::Error: std::fmt::Debug,
    <Codec as codee::Decoder<ServerType>>::Error: std::fmt::Debug,
{
    type Error =
        Error<<Codec as Encoder<ClientType>>::Error, <Codec as Decoder<ServerType>>::Error>;
    type Sender = Sender<Codec, ClientType, ServerType>;
    type Receiver = Receiver<Codec, ClientType, ServerType>;

    async fn establish(url: &Url) -> Result<(Self::Sender, Self::Receiver), Self::Error> {
        let (sink, stream) = gloo_net::websocket::futures::WebSocket::open(url.as_str())?.split();
        Ok((
            Sender {
                sink,
                _types: PhantomData,
            },
            Receiver {
                stream,
                _types: PhantomData,
            },
        ))
    }
}

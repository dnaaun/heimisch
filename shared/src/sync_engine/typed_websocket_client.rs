use futures::{Sink, Stream};
use std::fmt::Debug;
use url::Url;

/// An error to distinguish between socket being closed and other errors.
#[derive(Debug)]
pub enum TWSError<E> {
    Closed,
    Actual(E),
}

/// Note that this defintion implies we are delegating ping-ponging tgo the implementation of the
/// traits.
pub trait TypedWebsocketClient<ClientType, ServerType, Codec> {
    type Error: Debug;
    type Sender: Sink<ClientType, Error = TWSError<Self::Error>>;
    type Receiver: Stream<Item = Result<ServerType, TWSError<Self::Error>>>;

    #[allow(async_fn_in_trait)]
    async fn establish(url: &Url) -> Result<(Self::Sender, Self::Receiver), Self::Error>;
}

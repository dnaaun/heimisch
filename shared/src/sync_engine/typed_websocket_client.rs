use futures::{Sink, Stream};
use std::fmt::Debug;
use url::Url;

/// Note that this defintion implies we are delegating ping-ponging tgo the implementation of the
/// traits.
pub trait TypedWebsocketClient<ClientType, ServerType, Codec> {
    type Error: Debug;
    type Sender: Sink<ClientType, Error = Self::Error>;
    type Receiver: Stream<Item = Result<ServerType, Self::Error>>;

    #[allow(async_fn_in_trait)]
    async fn establish(url: &Url) -> Result<(Self::Sender, Self::Receiver), Self::Error>;
}

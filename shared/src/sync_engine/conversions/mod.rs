pub mod conversion_error;

pub mod github_api;
mod to_db;
pub mod webhooks;

use std::future::Future;

use futures::future::OptionFuture;
pub use to_db::*;

/// Instead of having to do `OptionFuture::from(some_thing.map(|x| x.future_creation())).await`, I
/// can just do `some_thing.map_to_future(|x| x.future_creation()).await`.
trait MapToFuture<T> {
    async fn map_to_future<Fut>(self, f: impl FnOnce(T) -> Fut) -> Option<Fut::Output>
    where
        Fut: Future;
}

impl<T> MapToFuture<T> for Option<T> {
    async fn map_to_future<Fut>(self, f: impl FnOnce(T) -> Fut) -> Option<Fut::Output>
    where
        Fut: Future,
    {
        OptionFuture::from(self.map(f)).await
    }
}

/// This project shares a lot of code between stuff that's supposed to run
/// in the browser, in a CLI/daemon, and in a server.
/// Nothing needs to be Send/Sync in the browser because barring web workers
/// (which we don't use on this project), JS on the browser is single-threaded.
/// And indeed, structs and futures in the `idb` and (the wasm version of) `reqwest` crate
/// are not Send/Sync.
/// But, I do use those structs in abstractions that I do need to be
/// Send/Sync (when not running on the browser).
/// Particularly, I have some traits (like `RawDbTrait` and `TransportTrait`)
/// that have async methods (ie, methods that return futures)
/// must be Send+Sync if I want to use them in multithreaded environments.
/// So that means I have to do one of the following:
///
/// 1. Make them Send+Sync, and fool the Rust compiler into thinking that
///    the browser-specific stuff is Send+Sync (it will never run out of the browser
///    anyways),
/// 2. Do feature flag galore and have different versions of the traits
///    that are not Send+Sync for the browser-specific stuff.
/// 3. Use a single-threaded environment outside the browser as well.
///
/// I went with number 1 because it's lower-overhead for development.
/// 2 is perhaps the (slightly) safer option. But again, I think it's actually
/// kinda hard to create multiple threads in the browser. So I can just
/// take a closer look at this if I ever go for web workers for some reason
/// (and I don't think I'll ever want/need to).
#[pin_project::pin_project]
#[derive(Debug, Clone)]
pub struct JustSend<T>(#[pin] T);

impl<T> From<T> for JustSend<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> JustSend<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }

    pub fn take(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for JustSend<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for JustSend<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "hydrate")]
unsafe impl<T> Send for JustSend<T> {}

#[cfg(feature = "hydrate")]
unsafe impl<T> Sync for JustSend<T> {}

impl<F: std::future::Future> std::future::Future for JustSend<F> {
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.project().0.poll(cx)
    }
}

impl<S: futures::Stream> futures::Stream for JustSend<S> {
    type Item = S::Item;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().0.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

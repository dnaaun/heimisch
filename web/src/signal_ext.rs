use leptos::prelude::*;

pub trait ResultSignalExt<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
{
    fn transpose(self) -> Result<Signal<T>, Signal<E>>;
}

impl<T, E> ResultSignalExt<T, E> for Signal<Result<T, E>>
where
    T: Clone + Send + Sync + 'static + std::fmt::Debug,
    E: Clone + Send + Sync + 'static + std::fmt::Debug,
{
    fn transpose(self) -> Result<Signal<T>, Signal<E>> {
        match *self.read() {
            Ok(_) => Ok(Signal::derive(move || self.get().expect(""))),
            Err(_) => Err(Signal::derive(move || self.get().expect_err(""))),
        }
    }
}

pub trait OptionSignalExt<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn transpose(self) -> Option<Signal<T>>;
}

impl<T> OptionSignalExt<T> for Signal<Option<T>>
where
    T: Clone + Send + Sync + 'static,
{
    fn transpose(self) -> Option<Signal<T>> {
        self.read()
            .as_ref()
            .map(|_| Signal::derive(move || self.get().expect("")))
    }
}

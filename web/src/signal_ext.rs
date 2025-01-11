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
    T: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
{
    fn transpose(self) -> Result<Signal<T>, Signal<E>> {
        let ok_signal = Signal::derive(move || self.get().ok());
        let err_signal = Signal::derive(move || self.get().err());

        match *self.read() {
            Ok(_) => Ok(Signal::derive(move || ok_signal.get().expect(""))),
            Err(_) => Err(Signal::derive(move || err_signal.get().expect(""))),
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
        let this = self.clone();
        match *self.read() {
            Some(_) => Some(Signal::derive(move || this.get().expect(""))),
            None => None,
        }
    }
}

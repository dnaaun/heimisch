use leptos::prelude::*;

pub trait SignalLikeResultExt<T, E>
where
    T: Send + Sync + 'static + Clone,
    E: Send + Sync + 'static + Clone,
{
    type MySelf<S>
    where
        S: Send + Sync + 'static + Clone;

    fn transpose(self) -> Result<Self::MySelf<T>, Self::MySelf<E>>;
}

impl<T, E> SignalLikeResultExt<T, E> for Signal<Result<T, E>>
where
    T: Send + Sync + 'static + Clone + std::fmt::Debug,
    E: Send + Sync + 'static + Clone + std::fmt::Debug,
{
    type MySelf<S> = Signal<S>
    where
        S: Send + Sync + 'static + Clone;

    #[track_caller]
    fn transpose(self) -> Result<Self::MySelf<T>, Self::MySelf<E>> {
        match *self.read() {
            Ok(_) => Ok(Signal::derive(move || self.get().expect(""))),
            Err(_) => Err(Signal::derive(move || self.get().expect_err(""))),
        }
    }
}

impl<T, E> SignalLikeResultExt<T, E> for Memo<Result<T, E>>
where
    T: Send + Sync + 'static + Clone + std::fmt::Debug + PartialEq,
    E: Send + Sync + 'static + Clone + std::fmt::Debug + PartialEq,
{
    type MySelf<S> = Memo<S>
    where
        S: Send + Sync + 'static + Clone;

    #[track_caller]
    fn transpose(self) -> Result<Self::MySelf<T>, Self::MySelf<E>> {
        match *self.read() {
            Ok(_) => Ok(Memo::new(move |_| self.get().expect(""))),
            Err(_) => Err(Memo::new(move |_| self.get().expect_err(""))),
        }
    }
}

pub trait SignalLikeOptionExt<T>
where
    T: Clone + Send + Sync + 'static,
{
    type MySelf<S>
    where
        S: Send + Sync + 'static + Clone;

    #[track_caller]
    fn transpose(self) -> Option<Self::MySelf<T>>;
}

impl<T> SignalLikeOptionExt<T> for Signal<Option<T>>
where
    T: Clone + Send + Sync + 'static,
{
    type MySelf<S> = Signal<S>
    where
        S: Send + Sync + 'static + Clone;

    #[track_caller]
    fn transpose(self) -> Option<Self::MySelf<T>> {
        self.read()
            .as_ref()
            .map(|_| Signal::derive(move || self.get().expect("")))
    }
}

impl<T> SignalLikeOptionExt<T> for Memo<Option<T>>
where
    T: Clone + Send + Sync + 'static + PartialEq,
{
    type MySelf<S> = Memo<S>
    where
        S: Send + Sync + 'static + Clone;

    #[track_caller]
    fn transpose(self) -> Option<Self::MySelf<T>> {
        self.read()
            .as_ref()
            .map(|_| Memo::new(move |_| self.get().expect("")))
    }
}
use crate::{chain::Chain, Store};

#[marker]
pub trait StoreMarker<S> {}

impl<Head, Tail, S> StoreMarker<S> for Chain<Head, Tail> where Tail: StoreMarker<S> {}

impl<Head, Tail, S> StoreMarker<S> for Chain<Head, Tail> where Head: StoreMarker<S> {}

impl<S: Store> StoreMarker<S> for S::Marker {}

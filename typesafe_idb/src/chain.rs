use std::marker::PhantomData;

pub struct Chain<Head, Tail> {
    head: PhantomData<Head>,
    tail: PhantomData<Tail>,
}
impl<H, T> Chain<H, T> {
    pub fn link<HNew>(self) -> Chain<HNew, Chain<H, T>> {
        Chain {
            head: PhantomData,
            tail: PhantomData,
        }
    }
}

impl Chain<(), ()> {
    pub fn new<H, T>() -> Chain<H, T> {
        Chain {
            head: PhantomData,
            tail: PhantomData,
        }
    }
}

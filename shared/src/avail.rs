use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Avail<T> {
    Yes(T),

    #[default]
    No,
}

#[derive(Debug)]
pub struct NotAvailableError;

impl<T> Avail<T> {
    pub fn from_option(opt: Option<T>) -> Self {
        match opt {
            Some(i) => Avail::Yes(i),
            None => Avail::No,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Avail::Yes(i) => i,
            Avail::No => panic!("Avail is No"),
        }
    }

    /// I like `assume` rather than `unwrap_or` because the semantics is that the value is not
    /// present and we are _assuming_ that it is whatever we pass here.
    pub fn assume(self, t: T) -> T {
        match self {
            Avail::Yes(i) => i,
            Avail::No => t,
        }
    }

    pub fn map<O>(self, func: impl FnOnce(T) -> O) -> Avail<O> {
        match self {
            Avail::Yes(t) => Avail::Yes(func(t)),
            Avail::No => Avail::No,
        }
    }

    pub fn as_ref(&self) -> Avail<&T> {
        match self {
            Avail::Yes(t) => Avail::Yes(t),
            Avail::No => Avail::No,
        }
    }

    pub fn as_mut(&mut self) -> Avail<&mut T> {
        match self {
            Avail::Yes(t) => Avail::Yes(t),
            Avail::No => Avail::No,
        }
    }

    pub fn map_ref<O>(&self, func: impl FnOnce(&T) -> O) -> Avail<O> {
        match self {
            Avail::Yes(t) => Avail::Yes(func(t)),
            Avail::No => Avail::No,
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            Avail::Yes(i) => Some(i),
            Avail::No => None,
        }
    }

    pub fn to_result(self) -> Result<T, NotAvailableError> {
        match self {
            Avail::Yes(i) => Ok(i),
            Avail::No => Err(NotAvailableError),
        }
    }


    pub fn ok_or_else<E>(self, func: impl Fn() -> E) -> Result<T, E> {
        match self {
            Avail::Yes(i) => Ok(i),
            Avail::No => Err(func()),
        }
    }

    pub fn with_merged(self, right: Avail<T>) -> Result<Avail<T>, MergeError> {
        Ok(match self {
            left @ Avail::Yes(_) => left,
            Avail::No => right,
        })
    }

    pub fn merge(&mut self, right: Avail<T>) -> Result<(), MergeError> {
        match self {
            Avail::Yes(_) => match right {
                Avail::Yes(right) => *self = Avail::Yes(right),
                Avail::No => (),
            },
            Avail::No => *self = right,
        };

        Ok(())
    }
}

pub trait MergeStructWithAvails: Sized {
    fn merge(&mut self, other: Self) -> Result<(), MergeError>;
    fn with_merged(self, other: Self) -> Result<Self, MergeError>;
}

pub type AvailIntoIter<T> = impl Iterator<Item = T>;

impl<T> IntoIterator for Avail<T> {
    type Item = T;

    type IntoIter = AvailIntoIter<T>;

    #[define_opaque(AvailIntoIter)]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Avail::Yes(i) => Some(i).into_iter(),
            Avail::No => None.into_iter(),
        }
    }
}

impl<T> From<T> for Avail<T> {
    fn from(value: T) -> Self {
        Avail::Yes(value)
    }
}

#[derive(Debug)]
pub enum MergeError {
    NonMatchingAttr(&'static str),
}

#[cfg(feature = "hydrate")]
mod a {
    use crate::avail::Avail;
    use leptos::either::Either;
    use leptos::prelude::Render;

    use leptos::tachys::view::iterators::OptionState;
    use leptos::tachys::view::Position;

    impl<T> leptos::prelude::AddAnyAttr for Avail<T>
    where
        T: leptos::prelude::AddAnyAttr,
    {
        type Output<SomeNewAttr: leptos::attr::Attribute> =
            Option<<T as leptos::prelude::AddAnyAttr>::Output<SomeNewAttr>>;

        fn add_any_attr<NewAttr: leptos::attr::Attribute>(
            self,
            attr: NewAttr,
        ) -> Self::Output<NewAttr>
        where
            Self::Output<NewAttr>: leptos::prelude::RenderHtml,
        {
            self.map(|n| n.add_any_attr(attr)).to_option()
        }
    }
    impl<T> Render for Avail<T>
    where
        T: Render,
    {
        type State = OptionState<T>;

        fn build(self) -> Self::State {
            match self {
                Self::Yes(value) => Either::Left(value),
                Self::No => Either::Right(()),
            }
            .build()
        }

        fn rebuild(self, state: &mut Self::State) {
            match self {
                Self::Yes(value) => Either::Left(value),
                Self::No => Either::Right(()),
            }
            .rebuild(state)
        }
    }

    #[cfg(feature = "hydrate")]
    impl<T> leptos::prelude::RenderHtml for Avail<T>
    where
        T: leptos::prelude::RenderHtml,
    {
        type AsyncOutput = Option<T::AsyncOutput>;

        const MIN_LENGTH: usize = T::MIN_LENGTH;

        fn dry_resolve(&mut self) {
            if let Avail::Yes(inner) = self.as_mut() {
                inner.dry_resolve();
            }
        }

        async fn resolve(self) -> Self::AsyncOutput {
            match self {
                Self::No => None,
                Self::Yes(value) => Some(value.resolve().await),
            }
        }

        fn html_len(&self) -> usize {
            match self {
                Avail::Yes(i) => i.html_len() + 3,
                Avail::No => 3,
            }
        }

        fn to_html_with_buf(
            self,
            buf: &mut String,
            position: &mut leptos::tachys::view::Position,
            escape: bool,
            mark_branches: bool,
        ) {
            match self {
                Avail::Yes(value) => leptos::either::Either::Left(value),
                Avail::No => leptos::either::Either::Right(()),
            }
            .to_html_with_buf(buf, position, escape, mark_branches)
        }

        fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
            self,
            buf: &mut leptos::tachys::ssr::StreamBuilder,
            position: &mut Position,
            escape: bool,
            mark_branches: bool,
        ) where
            Self: Sized,
        {
            match self {
                Avail::Yes(value) => leptos::either::Either::Left(value),
                Avail::No => leptos::either::Either::Right(()),
            }
            .to_html_async_with_buf::<OUT_OF_ORDER>(
                buf,
                position,
                escape,
                mark_branches,
            )
        }

        #[track_caller]
        fn hydrate<const FROM_SERVER: bool>(
            self,
            cursor: &leptos::tachys::hydration::Cursor,
            position: &leptos::tachys::view::PositionState,
        ) -> Self::State {
            match self {
                Avail::Yes(value) => leptos::either::Either::Left(value),
                Avail::No => leptos::either::Either::Right(()),
            }
            .hydrate::<FROM_SERVER>(cursor, position)
        }
    }
}

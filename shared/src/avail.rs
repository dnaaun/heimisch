use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum Avail<T> {
    Yes(T),

    #[default]
    No,
}

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

    pub fn unwrap_or(self, t: T) -> T {
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
            Avail::Yes(t) => Avail::Yes(&t),
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

pub type AvailIntoIter<T> = impl Iterator<Item = T>;

impl<T> IntoIterator for Avail<T> {
    type Item = T;

    type IntoIter = AvailIntoIter<T>;

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

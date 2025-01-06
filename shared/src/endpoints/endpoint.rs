use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

pub trait QueryParams {
    fn get_pairs(&self) -> impl Iterator<Item = (&str, &str)>;
}

impl QueryParams for HashMap<String, String> {
    fn get_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

impl QueryParams for () {
    fn get_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        std::iter::empty()
    }
}

impl<T: QueryParams> QueryParams for Option<T> {
    fn get_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        match self {
            Some(t) => Box::new(t.get_pairs()) as Box<dyn Iterator<Item = (&str, &str)>>,
            None => Box::new(std::iter::empty()),
        }
    }
}

pub enum Method<P = ()> {
    Post { payload: P },
    Get,
}

pub trait PostEndpoint {
    type QueryParams: Serialize + DeserializeOwned + Send + 'static;
    const PATH: &'static str;
    type JsonResponse: Serialize + DeserializeOwned;
    type JsonPayload: Serialize + DeserializeOwned;
}

pub trait GetEndpoint {
    type QueryParams: Serialize + DeserializeOwned + Send + 'static;
    const PATH: &'static str;
    type JsonResponse: Serialize + DeserializeOwned;
}

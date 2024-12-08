use http::{HeaderName, HeaderValue, Method};
use reqwest::Url;
use std::fmt::Debug;
use wiremock::{
    matchers::{self, path, query_param},
    Mock, Respond,
};

// TODO: Flag everything wiremock behind `cfg(test)`.

enum BuilderInner {
    Ok(reqwest::RequestBuilder, wiremock::MockBuilder),
    Err(reqwest::Error),
}

impl BuilderInner {
    fn map(
        self,
        func: impl FnOnce(
            reqwest::RequestBuilder,
            wiremock::MockBuilder,
        )
            -> Result<(reqwest::RequestBuilder, wiremock::MockBuilder), reqwest::Error>,
    ) -> Self {
        match self {
            BuilderInner::Ok(request_builder, mock_builder) => {
                match func(request_builder, mock_builder) {
                    Ok((request_builder, mock_builder)) => {
                        BuilderInner::Ok(request_builder, mock_builder)
                    }
                    Err(err) => BuilderInner::Err(err),
                }
            }
            BuilderInner::Err(e) => BuilderInner::Err(e),
        }
    }
}

pub struct Builder {
    inner: BuilderInner,
}

impl Builder {
    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error> + Debug,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error> + Debug,
        K: Clone,
        V: Clone,
    {
        let inner = self.inner.map(move |request, mock| {
            Ok((
                request.header(key.clone(), value.clone()),
                mock.and(matchers::header(key, value)),
            ))
        });

        Self { inner }
    }

    pub fn build(self) -> reqwest::Result<reqwest::Request> {
        match self.inner {
            BuilderInner::Ok(request_builder, _) => request_builder.build(),
            BuilderInner::Err(error) => Err(error),
        }
    }

    pub async fn send(self) -> reqwest::Result<reqwest::Response> {
        match self.inner {
            BuilderInner::Ok(request_builder, _) => request_builder.send().await,
            BuilderInner::Err(error) => Err(error),
        }
    }

    pub fn respond_with<R>(self, responder: R) -> Result<Mock, reqwest::Error>
    where
        R: Respond + 'static,
    {
        match self.inner {
            BuilderInner::Ok(_, mock_builder) => Ok(mock_builder.respond_with(responder)),
            BuilderInner::Err(error) => Err(error),
        }
    }
}

pub struct ClientWithMock {
    client: reqwest::Client,
}

impl ClientWithMock {
    pub fn request(&self, method: Method, url: Url) -> Builder {
        let request = self.client.request(method, url.clone());
        let mock = Mock::given(path(url.path()));
        let mock = url
            .query_pairs()
            .fold(mock, |acc, (query_key, query_value)| {
                acc.and(query_param(query_key, query_value))
            });

        Builder {
            inner: BuilderInner::Ok(request, mock),
        }
    }

    pub fn post(&self, url: Url) -> Builder {
        self.request(Method::POST, url)
    }

    pub fn get(&self, url: Url) -> Builder {
        self.request(Method::GET, url)
    }
}

pub trait GetClientWithMock {
    fn with_mock(&self) -> ClientWithMock;
}

impl GetClientWithMock for reqwest::Client {
    fn with_mock(&self) -> ClientWithMock {
        ClientWithMock {
            client: self.clone(),
        }
    }
}

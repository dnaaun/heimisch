use http::{HeaderName, HeaderValue, Method};
use reqwest::Url;
use std::fmt::Debug;

#[cfg(feature = "mocking")]
use wiremock::{
    matchers::{self, path, query_param},
    Mock, Respond,
};

// NOTE: Flag everything wiremock behind `cfg(test)`.

struct Inner {
    request: reqwest::RequestBuilder,

    #[cfg(feature = "mocking")]
    mock: wiremock::MockBuilder,
}

pub struct Builder {
    inner: Result<Inner, reqwest::Error>,
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
        let inner = self.inner.map(
            |Inner {
                 request,

                 #[cfg(feature = "mocking")]
                 mock,
             }| Inner {
                request: request.header(key.clone(), value.clone()),

                #[cfg(feature = "mocking")]
                mock: mock.and(matchers::header(key, value)),
            },
        );

        Self { inner }
    }

    pub fn build(self) -> reqwest::Result<reqwest::Request> {
        self.inner?.request.build()
    }

    pub async fn send(self) -> reqwest::Result<reqwest::Response> {
        self.inner?.request.send().await
    }

    #[cfg(feature = "mocking")]
    pub fn respond_with<R>(self, responder: R) -> Result<Mock, reqwest::Error>
    where
        R: Respond + 'static,
    {
        Ok(self.inner?.mock.respond_with(responder))
    }
}

pub struct ClientWithMock {
    client: reqwest::Client,
}

impl ClientWithMock {
    pub fn request(&self, method: Method, url: Url) -> Builder {
        let request = self.client.request(method, url.clone());

        #[cfg(feature = "mocking")]
        let mock = {
            let mock = Mock::given(path(url.path()));
            url.query_pairs()
                .fold(mock, |acc, (query_key, query_value)| {
                    acc.and(query_param(query_key, query_value))
                })
        };

        Builder {
            inner: Ok(Inner {
                #[cfg(feature = "mocking")]
                mock,
                request,
            }),
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

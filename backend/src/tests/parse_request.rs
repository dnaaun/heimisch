use bytes::Bytes;
use http::Method;
use std::str::FromStr;
use std::{collections::HashMap, path::Path};
use tokio::io::AsyncReadExt;

use axum_test::{TestResponse, TestServer};
use tokio::fs::File;

use super::{TestErrorSource, TestResult};

#[derive(Debug, derive_more::Display)]
struct HttpParsingError(String);

impl std::error::Error for HttpParsingError {}

#[derive(Clone)]
pub struct ParsedHttpRequest {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Bytes,
}

impl ParsedHttpRequest {
    pub async fn from_file(path: &Path) -> TestResult<Self> {
        use TestErrorSource::*;
        let mut file = File::open(path)
            .await
            .map_err(|e| ParseRequestFile(e.into()))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .await
            .map_err(|e| ParseRequestFile(e.into()))?;

        let mut lines = buf.lines();
        let first_line = lines
            .next()
            .ok_or_else(|| ParseRequestFile(HttpParsingError("no first line".into()).into()))?;
        let mut first_line_parts = first_line.split(" ");
        let method_str = first_line_parts.next().ok_or_else(|| {
            ParseRequestFile(HttpParsingError("first line malformed".into()).into())
        })?;
        let path = first_line_parts
            .next()
            .ok_or_else(|| {
                ParseRequestFile(HttpParsingError("first line malformed".into()).into())
            })?
            .to_owned();

        let method = Method::from_str(method_str).map_err(|_| {
            ParseRequestFile(HttpParsingError("method not supported/valid".into()).into())
        })?;

        let version = first_line_parts.next().ok_or_else(|| {
            ParseRequestFile(HttpParsingError("first line malformed".into()).into())
        })?;
        if version != "HTTP/1.1" {
            return Err(
                ParseRequestFile(HttpParsingError("version not HTTP/1.1".into()).into()).into(),
            );
        }

        let mut headers: HashMap<_, _> = Default::default();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }

            let mut parts = line.splitn(2, ": ");
            let key = parts
                .next()
                .ok_or_else(|| {
                    ParseRequestFile(HttpParsingError("header line malformed".into()).into())
                })?
                .to_lowercase();
            let value = parts.next().ok_or_else(|| {
                ParseRequestFile(HttpParsingError("header line malformed".into()).into())
            })?;

            headers.insert(key, value.to_owned());
        }

        let body = lines
            .remainder()
            .map(|t| Bytes::from(t.as_bytes().to_vec()))
            .unwrap_or_default();

        Ok(Self {
            method,
            path,
            headers,
            body,
        })
    }

    pub async fn make(self, test_server: &TestServer) -> TestResponse {
        let Self {
            method,
            path,
            headers,
            body,
        } = self;
        let req = test_server.method(method, &path).bytes(body);

        let req = headers
            .into_iter()
            .fold(req, |acc, (k, v)| acc.add_header(k, v));

        req.await
    }
}

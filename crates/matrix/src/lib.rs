//! This library deals with forwarding Matrix requests to the server.
//! Comments have been used sparingly as the specification contains all the technical details.

//! We rely on `ruma` to abstract away the boilerplate introduced by HTTP requests,
//! without sacrificing flexibility by defining our own request and response types.
//!
//! reference: https://docs.ruma.io/ruma_common/api/index.html

pub mod admin;
pub mod client;

use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use ruma_client::{DefaultConstructibleHttpClient, HttpClient, HttpClientExt};

#[allow(unused_imports)]
use ruma_common::api::error::MatrixError as Error;

#[derive(Debug)]
pub struct Handle {
    inner: reqwest::Client,
}

impl Handle {
    pub async fn new() {
      self.send_matrix_request(, access_token, for_versions, request)
    }

    pub async fn dispatch(&self) {
      self.send_matrix_request(, access_token, for_versions, request)
    }
}

#[async_trait]
impl HttpClient for Handle {
    type RequestBody = BytesMut;
    type ResponseBody = Bytes;
    type Error = reqwest::Error;

    async fn send_http_request(
        &self,
        req: http::Request<BytesMut>,
    ) -> Result<http::Response<Bytes>, reqwest::Error> {
        let req = req.map(|body| body.freeze()).try_into()?;
        let mut res = self.inner.execute(req).await?;

        let mut http_builder = http::Response::builder()
            .status(res.status())
            .version(res.version());
        std::mem::swap(
            http_builder
                .headers_mut()
                .expect("http::response::Builder to be usable"),
            res.headers_mut(),
        );

        Ok(http_builder
            .body(res.bytes().await?)
            .expect("http::Response construction to work"))
    }
}

impl DefaultConstructibleHttpClient for Handle {
    fn default() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }
}

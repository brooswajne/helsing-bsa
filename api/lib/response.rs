//! Structs which implement [lambda_http::IntoResponse] for easy re-use.

use std::future::Future;
use std::pin::Pin;

use lambda_http::{Body, IntoResponse, Response};

/// A response type which just sends raw text to the client, with content type
/// `text/plain`.
///
/// This is useful because the default implementation of [IntoResponse] for a
/// string is to use the content type `application/json`.
pub struct PlainTextResponse(String);

impl PlainTextResponse {
    pub fn new(text: String) -> Self {
        Self(text)
    }
}

impl From<String> for PlainTextResponse {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}
impl From<&str> for PlainTextResponse {
    fn from(string: &str) -> Self {
        Self::new(string.to_owned())
    }
}

impl IntoResponse for PlainTextResponse {
    fn into_response(self) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> {
        let body = Body::Text(self.0);
        let response = Response::builder()
            .status(200)
            .header("Content-Type", "text/plain")
            .body(body)
            .expect("we control all arguments to the builder");
        Box::pin(std::future::ready(response))
    }
}

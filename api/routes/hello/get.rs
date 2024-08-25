//! A basic "Hello world" route, just for example's sake.

use lambda_http::{Error, IntoResponse, Request, Response};
use lib::greet::greet;

async fn handler(request: Request) -> Result<impl IntoResponse, Error> {
    println!("Received: {request:?}");
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(greet("world"))?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(handler)).await
}

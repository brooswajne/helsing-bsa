//! The root route for the entire API. Doesn't do anything fancy.

use lambda_http::{Error, IntoResponse, Request, Response};

async fn handler(request: Request) -> Result<impl IntoResponse, Error> {
    println!("Received: {request:?}");
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body("This is the index page.".to_owned())?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(handler)).await
}

//! A route which greets the user (based on a provided path parameter).

use lambda_http::{Error, IntoResponse, Request, RequestExt, Response};

async fn handler(request: Request) -> Result<impl IntoResponse, Error> {
    println!("Received: {request:?}");

    let path_parameters = request.path_parameters();
    let user = path_parameters
        .first("user")
        .ok_or("Request is missing a \"user\" path parameter")?;

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(format!("Hello, {user}!"))?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(handler)).await
}

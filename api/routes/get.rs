//! The root route for the entire API. Doesn't do anything fancy.

use lib::get_api_version_string;
use lib::macros::route_handler;
use lib::response::PlainTextResponse;

#[route_handler]
async fn index() -> PlainTextResponse {
    let api_version = get_api_version_string();

    let runtime_info = lambda_runtime::Config::from_env();

    format!(
        "{api_version}\n\n\
        Deployment Details\n\
        ------------------\n\
        Function Name: {:?}\n\
        Function Version: {:?}\n\
        Function Memory: {}",
        runtime_info.function_name, runtime_info.version, runtime_info.memory,
    )
    .into()
}

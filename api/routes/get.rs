//! The root route for the entire API. Doesn't do anything fancy.

use lib::macros::route_handler;
use lib::response::PlainTextResponse;

#[route_handler]
async fn index() -> PlainTextResponse {
    "This is the index page.".into()
}

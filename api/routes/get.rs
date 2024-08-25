//! The root route for the entire API. Doesn't do anything fancy.

use lib::macros::route_handler;

#[route_handler]
async fn index() -> String {
    "This is the index page.".to_string()
}

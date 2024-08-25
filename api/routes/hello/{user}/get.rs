//! A route which greets the user (based on a provided path parameter).

use lib::greet::create_greeting;
use lib::macros::route_handler;

#[route_handler]
async fn greet(user: String) -> String {
    create_greeting(&user)
}

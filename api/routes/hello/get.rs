//! A basic "Hello world" route, just for example's sake.

use lib::greet::create_greeting;
use lib::macros::route_handler;
use lib::response::PlainTextResponse;

#[route_handler]
async fn greet_the_world() -> PlainTextResponse {
    create_greeting("world").into()
}

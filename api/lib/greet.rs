//! A stupid example of shared code used by multiple routes.

pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

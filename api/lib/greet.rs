//! A stupid example of shared code used by multiple routes.

pub fn create_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

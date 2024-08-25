//! Contains shared code and utilities for use throughout our different
//! endpoints.

use git_version::git_version;
pub use macros;

pub mod greet;
pub mod response;

pub fn get_api_version_string() -> String {
    let api_version = git_version!(args = ["--tags", "--always"]);
    let rustc_version = rustc_version_runtime::version();
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    format!("API version: {api_version}\nbuilt with rustc {rustc_version} ({os}/{arch})")
}

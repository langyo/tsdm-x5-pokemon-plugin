#[cfg(target_arch = "wasm32")]
pub mod config;

#[cfg(target_arch = "wasm32")]
pub mod dioxus;

#[cfg(target_arch = "wasm32")]
mod web_entry;

#[cfg(target_arch = "wasm32")]
pub use web_entry::*;

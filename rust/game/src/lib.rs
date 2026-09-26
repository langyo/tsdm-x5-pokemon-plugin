mod app;
pub mod components;
mod hooks;
mod pages;
pub mod prelude;
mod router;
pub mod state;
pub mod utils;
#[cfg(target_arch = "wasm32")]
mod web_entry;

#[cfg(target_arch = "wasm32")]
pub use web_entry::*;

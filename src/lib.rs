#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub use app::SC2ReplayAnalyser;
pub mod common;
pub use common::*;
pub mod details;
pub use details::*;
pub mod api;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

#[cfg(not(target_arch = "wasm32"))]
pub mod error;

#[cfg(not(target_arch = "wasm32"))]
pub use error::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub use app::SC2ReplayExplorer;
pub mod common;
pub use common::*;
pub mod api;
pub mod meta;

pub const DETAILS_IPC: &str = "details.ipc";

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

#[cfg(not(target_arch = "wasm32"))]
pub mod error;

#[cfg(not(target_arch = "wasm32"))]
pub use error::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

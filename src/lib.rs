#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::SC2ReplayAnalyser;
pub mod api;
pub mod cli;
pub mod common;
pub mod error;
pub mod server;
pub use common::*;
pub use error::*;

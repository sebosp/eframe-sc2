//! Handling of Error Messages
//!

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Polars Error: {0}")]
    Polars(#[from] polars::error::PolarsError),
    #[error("S2Protocol Error: {0}")]
    S2Protocol(#[from] s2protocol::S2ProtocolError),
    #[error("Axum Error: {0}")]
    Axum(#[from] axum::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Eframe Error: {0}")]
    Eframe(#[from] eframe::Error),
    #[error("UTF8 Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Other Error: {0}")]
    Other(String),
}

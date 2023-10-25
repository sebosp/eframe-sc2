//! Meta data response for http responses

use std::time::Instant;

use serde::{Deserialize, Serialize};

/// The status of the response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    /// The response was successful
    Ok,
    /// There was an error
    Error { message: String },
}

impl Default for ResponseStatus {
    fn default() -> Self {
        Self::Ok
    }
}

/// Contains the meta data of data frame results to be sent back to the clients.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ResponseMeta {
    /// Wether there was an error or not
    pub status: ResponseStatus,
    /// The total number of rows in the data frame
    pub total: usize,
    /// The time at which the snapshot was taken
    pub epoch: u64,
    /// The drutation of the operation in milliseconds
    pub duration: u64,
}

impl ResponseMeta {
    /// Creates a new ResponseMeta
    pub fn new(
        status: ResponseStatus,
        total: usize,
        epoch: u64,
        message: String,
        duration: u64,
    ) -> Self {
        Self {
            status,
            total,
            epoch,
            duration,
        }
    }

    /// Creates a new ResponseMeta with status "ok"
    pub fn ok(total: usize, epoch: u64, duration: u64) -> Self {
        Self {
            status: ResponseStatus::Ok,
            total,
            epoch,
            duration,
        }
    }
}

pub struct ResponseMetaBuilder {
    /// Wether there was an error or not
    pub status: ResponseStatus,
    /// The total number of rows in the data frame
    pub total: usize,
    /// The epoch at which the operation was performed
    pub epoch: u64,
    /// The drutation of the operation in milliseconds
    pub start: Instant,
}

impl ResponseMetaBuilder {
    /// Creates a new ResponseMetaBuilder
    pub fn new() -> Self {
        Self {
            status: ResponseStatus::Ok,
            total: 0,
            epoch: chrono::Utc::now().timestamp_millis() as u64,
            start: Instant::now(),
        }
    }

    /// Sets the total number of rows in the data frame
    pub fn with_total(mut self, total: usize) -> Self {
        self.total = total;
        self
    }

    /// Sets the total number of rows in the data frame
    pub fn with_error(mut self, error: impl ToString) -> Self {
        self.status = ResponseStatus::Error {
            message: error.to_string(),
        };
        self
    }

    /// Builds the ResponseMeta
    pub fn build(self) -> ResponseMeta {
        let duration = self.start.elapsed().as_millis() as u64;
        ResponseMeta {
            status: self.status,
            total: self.total,
            epoch: self.epoch,
            duration,
        }
    }
}

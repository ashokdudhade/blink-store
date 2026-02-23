//! Library-level errors for Blink-Store.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlinkError {
    #[error("key not found: {0}")]
    NotFound(String),

    #[error("storage is at capacity and eviction failed")]
    AtCapacity,

    #[error("internal error: {0}")]
    Internal(String),
}

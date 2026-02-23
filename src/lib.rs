//! Blink Store: in-memory key-value store with sampled eviction and memory-cap enforcement.

pub mod engine;
pub mod error;
pub mod logging;
pub mod protocol;
pub mod server;

pub use engine::{BlinkStorage, MemoryEngine};
pub use error::BlinkError;
pub use protocol::{parse_request, Command, Response};
pub use server::run_tcp;
#[cfg(unix)]
pub use server::run_unix;

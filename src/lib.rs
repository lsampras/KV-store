#![deny(missing_docs)]
//! # kvs
//!
//! `kvs` is a key value store
//! this lib is create to learn rust
mod store;
mod file_storage;

/// Information about all the errors & types emitted by KVStore
pub mod error;
/// database level commands
pub mod command;

/// Structured logging utils
pub mod logging;

/// Implementing common traits for storage engine
pub mod traits;

/// built-in storage engine for key-value pairs
pub use store::KvStore;
/// File storage utils for built-in key values
pub use file_storage::{StorageHandler, LogPointer};


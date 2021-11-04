// #![deny(missing_docs)]
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

pub mod logging;

pub use store::KvStore;
pub use file_storage::{StorageHandler, LogPointer};
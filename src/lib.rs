#![deny(missing_docs)]
//! # kvs
//!
//! `kvs` is a key value store
//! this lib is create to learn rust
mod store;

/// Information about all the errors & types emitted by KVStore
pub mod error;

pub use store::KvStore;
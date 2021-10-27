use std::io;
use failure::{Fail};

/// A type alias for using KVError as part of results
pub type KVResult<T> = Result<T, KVError>;

/// documentation
#[derive(Fail, Debug)]
pub enum KVError {
	/// IO errors
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
	/// Placeholder Error to represent string for dynamic error types
	/// TODO: Convert usages of this to static type errors
    #[fail(display = "{}", _0)]
    Default(String),
}
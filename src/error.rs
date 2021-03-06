use std::io;
use failure::{Fail};
use bson::{ser, de};
use std::sync::{Arc, PoisonError};
use std::convert::From;
/// A type alias for using KVError as part of results
pub type KVResult<T> = Result<T, KVError>;

/// documentation
#[derive(Fail, Debug)]
pub enum KVError {
	/// IO errors
    #[fail(display = "{}", _0)]
    Io(#[cause] Arc<io::Error>),
	/// serialization errors
    #[fail(display = "{}", _0)]
    Serialization(#[cause] ser::Error),
	/// deserialization errors
    #[fail(display = "{}", _0)]
    Deserialization(#[cause] de::Error),
	/// Placeholder Error to represent string for dynamic error types
	/// TODO: Convert usages of this to static type errors
    #[fail(display = "{}", _0)]
    Default(String),
}

impl PartialEq for KVError {
    fn eq(&self, other:&KVError) ->  bool{
        match self {
            KVError::Io(_) => matches!(*other, KVError::Io(_)),
            KVError::Default(i) => matches!(other, KVError::Default(j) if i == j),
            _ => true
        }
    }
}

impl From<io::Error> for KVError {
    fn from(error: io::Error) ->  Self{
        KVError::Io(Arc::new(error))
    }
}

impl From<de::Error> for KVError {
    fn from(error: de::Error) ->  Self{
        KVError::Deserialization(error)
    }
}

impl From<ser::Error> for KVError {
    fn from(error: ser::Error) ->  Self{
        KVError::Serialization(error)
    }
}

impl<T > From<PoisonError<T>> for KVError {
    fn from(error: PoisonError<T>) ->  KVError {
        KVError::Default(error.to_string())
    }
}
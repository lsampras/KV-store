use crate::error::{KVResult};

/// Public storage engine trait that would be used by the db server
pub trait KvsEngine {
	/// Get value for a stored string key
	/// The value are both expected to be strings
	/// return none incase of no key available
	fn get(&self, key: String) -> KVResult<Option<String>>;
	/// Set value for a key
	/// The key & values are both expected to be strings
	fn set(&mut self, key: String, value: String) -> KVResult<()>;
	/// Remove value for a key
	/// This will delete the value stored under a string
	fn remove(&mut self, key: String) -> KVResult<()>;
}
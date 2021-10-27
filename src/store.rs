use std::fs::File;
use std::{collections::HashMap};
use crate::error::{KVError, KVResult};
use crate::{get_log_file, write_command_to_file};
use crate::command::LogRecord;
/// KvStore handling the operations of a (persistent) key-value store
/// It allows you to set, get or delete keys in the underlying kv store
pub struct KvStore {
	memory_store: HashMap<String, String>,
	log_file: File
}

#[allow(unused)]
impl KvStore {
	/// Create a new KvStore
	/// the store can then be used to set, get or delete keys
	pub fn new() -> KVResult<Self> {
		Ok(KvStore {
			memory_store: HashMap::new(),
			log_file: get_log_file("foo2.txt".to_string())?
		})
	}

	/// Get the value of a key in a KvStore
	/// It expects a string key and will return an Option<String> for the stored value
	pub fn get(&self, key: String) -> KVResult<String> {
		match self.memory_store.get(&key) {
			Some(val_ref) => Ok(val_ref.to_owned()),
			None => Err(KVError::Default("Key not found".to_owned()))
		}
	}

	/// Add or update the value of an existing key
	/// Both key & value are expected to be strings
	/// failures are logged to stderr
	pub fn set(&mut self, key: String, value: String) -> KVResult<()>{
		self.memory_store.insert(key.clone(), value.clone());
		write_command_to_file(&mut self.log_file, LogRecord::Set(key, value))
	}

	/// clear a given key from kv store
	/// any errors will be logged to stderr
	pub fn remove(&mut self, key: String) -> KVResult<()>{
		self.memory_store.remove(&key);
		write_command_to_file(&mut self.log_file, LogRecord::Delete(key))
	}
}
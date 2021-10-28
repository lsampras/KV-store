use std::collections::HashMap;
use crate::error::{KVResult};
use crate::{LogPointer, StorageHandler};
use crate::command::LogRecord;
/// KvStore handling the operations of a (persistent) key-value store
/// It allows you to set, get or delete keys in the underlying kv store
#[derive(Debug)]
pub struct KvStore {
	index_store: HashMap<String, LogPointer>,
	storage: StorageHandler
}

#[allow(unused)]
impl KvStore {
	/// Create a new KvStore
	/// the store can then be used to set, get or delete keys
	pub fn new() -> KVResult<Self> {
		let mut storage = StorageHandler::new("foo2.txt".to_string())?;
		let mut index_map = HashMap::new();
		for (pointer, record) in storage.read_all_logs()?.iter() {
			let key = match &*record {
				LogRecord::Delete(key) => key,
				LogRecord::Set(key, val) => key
			};
			index_map.insert(key.to_owned(), *pointer);
		}
		Ok(KvStore {
			index_store: index_map,
			storage: storage
		})
	}

	/// Get the value of a key in a KvStore
	/// It expects a string key and will return an Option<String> for the stored value
	pub fn get(&self, key: String) -> KVResult<Option<String>> {
		let pointer = self.index_store.get(&key);
		match pointer {
			None => Ok(None),
			Some(p) => Ok(
				match self.storage.read_log_record_with_pointer(p)? {
					LogRecord::Delete(_) => None,
					LogRecord::Set(_, val) => Some(val)
				}
			)
		}
	}

	/// Add or update the value of an existing key
	/// Both key & value are expected to be strings
	/// failures are logged to stderr
	pub fn set(&mut self, key: String, value: String) -> KVResult<()>{
		let pointer = self.storage.write_record(LogRecord::Set(key.clone(), value))?;
		self.index_store.insert(key, pointer);
		Ok(())
	}

	/// clear a given key from kv store
	/// any errors will be logged to stderr
	pub fn remove(&mut self, key: String) -> KVResult<()>{
		let pointer = self.storage.write_record(LogRecord::Delete(key.clone()))?;
		self.index_store.insert(key, pointer);
		Ok(())
	}
}
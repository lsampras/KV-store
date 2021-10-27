use std::collections::HashMap;
use crate::error::{KVError, KVResult};
use crate::{StorageHandler};
use crate::command::LogRecord;
/// KvStore handling the operations of a (persistent) key-value store
/// It allows you to set, get or delete keys in the underlying kv store
pub struct KvStore {
	memory_store: HashMap<String, String>,
	storage: StorageHandler
}

#[allow(unused)]
impl KvStore {
	/// Create a new KvStore
	/// the store can then be used to set, get or delete keys
	pub fn new() -> KVResult<Self> {
		let mut storage = StorageHandler::new("foo2.txt".to_string())?;
		let mut hashmap = HashMap::new();
		if storage.is_read_pending() {
			let mut reader = storage.get_log_reader();
			let mut log_iterator = reader.lock().unwrap();
			loop {
				match log_iterator.next() {
					Some(val) => {
						match val {
							Ok(LogRecord::Delete(k)) => {hashmap.remove(&k);},
							Ok(LogRecord::Set(k, v)) => {hashmap.insert(k, v);},
							Err(i) => {println!("{:?}", i);}
						}
					},
					None => {break;}
				}
			}
		}
		Ok(KvStore {
			memory_store: hashmap,
			storage: storage
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
		self.storage.write_record(LogRecord::Set(key, value))?;
		Ok(())
	}

	/// clear a given key from kv store
	/// any errors will be logged to stderr
	pub fn remove(&mut self, key: String) -> KVResult<()>{
		self.memory_store.remove(&key);
		self.storage.write_record(LogRecord::Delete(key))?;
		Ok(())
	}
}
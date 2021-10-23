use std::{collections::HashMap};

/// KvStore handling the operations of a (persistent) key-value store
/// It allows you to set, get or delete keys in the underlying kv store
pub struct KvStore {
	store: HashMap<String, String>
}

#[allow(unused)]
impl KvStore {
	/// Create a new KvStore
	/// the store can then be used to set, get or delete keys
	pub fn new() -> Self {
		KvStore {
			store: HashMap::new()
		}
	}

	/// Get the value of a key in a KvStore
	/// It expects a string key and will return an Option<String> for the stored value
	pub fn get(&self, key: String) -> Option<String> {
		match self.store.get(&key) {
			Some(val_ref) => Some(val_ref.to_owned()),
			None => None
		}
	}

	/// Add or update the value of an existing key
	/// Both key & value are expected to be strings
	/// failures are logged to stderr
	pub fn set(&mut self, key: String, value: String) {
		match self.store.insert(key.clone(), value.clone()) {
			None => eprintln!("Saving key {}  value {} failed", key, value),
			_ => ()
		};
	}

	/// clear a given key from kv store
	/// any errors will be logged to stderr
	pub fn remove(&mut self, key: String) {
		match self.store.remove(&key) {
			None => eprintln!("Removing key {}  failed", key),
			_ => ()
		};
	}
}
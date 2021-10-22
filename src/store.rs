use std::{collections::HashMap};

pub struct KvStore {
	store: HashMap<String, String>
}

#[allow(unused)]
impl KvStore {
	pub fn new() -> Self {
		KvStore {
			store: HashMap::new()
		}
	}

	pub fn get(&self, key: String) -> Option<String> {
		match self.store.get(&key) {
			Some(val_ref) => Some(val_ref.to_owned()),
			None => None
		}
	}

	pub fn set(&mut self, key: String, value: String) {
		match self.store.insert(key.clone(), value.clone()) {
			None => eprintln!("Saving key {}  value {} failed", key, value),
			_ => ()
		};
	}

	pub fn remove(&mut self, key: String) {
		match self.store.remove(&key) {
			None => eprintln!("Removing key {}  failed", key),
			_ => ()
		};
	}
}
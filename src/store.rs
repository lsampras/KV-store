pub struct KvStore;

#[allow(unused)]
impl KvStore {
	pub fn new() -> Self {
		KvStore
	}

	pub fn get(&self, key: String) -> Option<String> {
		Some(key)
	}

	pub fn set(&mut self, key: String, value: String) {

	}

	pub fn remove(&mut self, key: String) {

	}
}
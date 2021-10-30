use serde::{Serialize, Deserialize};

/// Represents individual logs in files
#[derive(Serialize, Deserialize, Debug)]
pub enum LogRecord {
	/// Represents the command for setting a key
	#[serde(rename="s")]
	Set(String,String),
	/// Represents the command for deleting a key
	#[serde(rename="d")]
	Delete(String)
}

impl LogRecord {
	/// get key for a record
	pub fn get_key(&self) -> String {
		match &self {
			LogRecord::Delete(i) => i,
			LogRecord::Set(i, _) => i
		}.to_owned()
	}
}
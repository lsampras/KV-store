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
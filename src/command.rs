use serde::{Serialize, Deserialize};
use bson::{self, from_document, Document};
use std::io::Cursor;

use crate::error::{KVResult};

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

	pub fn from_bytes(bytes: &mut [u8]) -> KVResult<Self> {
		let mut reader = Cursor::new(bytes);
		Ok(from_document::<LogRecord>(Document::from_reader(&mut reader)?)?)
	}
}
use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use bson;

use crate::command::LogRecord;
use crate::error::KVResult;
use crate::error::KVError;

/// Create a struct with an enum which would handle all operations of files
/// this struct can maintain state & ownership of internal buffers


fn open_new_log_file(filename: String) -> KVResult<File> {
	OpenOptions::new()
				.create(true)
				.read(true)
				.write(true)
				.open(filename)
				.map_err(|err| KVError::from(err))
}

/// Get the file handle for log file which stores command logs
/// 
pub fn get_log_file(filename: String) -> KVResult<File> {
	OpenOptions::new().read(true).write(true).open("foo3.txt")
				// .and_then(|val| populate_index(val))
				.or_else(|err| 
					match err.kind() {
						io::ErrorKind::NotFound => open_new_log_file(filename),
						_ => Err(KVError::from(err))
					}
				)
}

// fn populate_index(file: &File, index: &mut HashMap<String, String>) {

// }

/// Write commands(LogRecord) to log file
/// TODO: use BufWriter here
pub fn write_command_to_file(file: &mut File, record: LogRecord) -> KVResult<()> {
	file.write(
		bson::to_vec(&record)
				.map_err(|err| KVError::Serialization(err))?
				.as_slice()
	).map_err(|err| KVError::from(err))?;
	Ok(())
}
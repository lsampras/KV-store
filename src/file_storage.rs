use std::io::{BufReader, BufWriter, ErrorKind};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::sync::{Arc, Mutex};
use bson;
use bson::{de, from_document, Document, to_vec};
use crate::command::LogRecord;
use crate::error::KVResult;
use crate::error::KVError;


/// Log Reader is an iterator over LogRecords that are read from the storage file
#[derive(Debug)]
pub struct LogReader {
	read_buf: BufReader<File>,
	log_reads_pending: bool
}

impl Iterator for LogReader {
	type Item = KVResult<LogRecord>;

	fn next(&mut self) -> Option<Self::Item> {
		match Document::from_reader(&mut self.read_buf) {
			Ok(doc) =>  Some(from_document::<LogRecord>(doc).map_err(|e| KVError::Deserialization(e))),
			Err(de::Error::Io(i)) => {
				if i.kind() == ErrorKind::UnexpectedEof {
					self.log_reads_pending = false;
					None
				} else {
					return Some(Err(KVError::Io(i)));
				}
			},
			Err(i) => Some(Err(KVError::Deserialization(i))),
		}
	}
}

/// Create a struct with an enum which would handle all operations of files
/// this struct can maintain state & ownership of internal buffers

pub struct StorageHandler {
	reader: Arc<Mutex<LogReader>>,
	writer: BufWriter<File>,
}

impl StorageHandler {
	/// create a storage handler for reading/writing logs
	/// This method takes in a filename (path) to be used as storage
	pub fn new(filename: String) -> KVResult<Self> {
		let mut log_reads_pending = true;
		let file = OpenOptions::new().read(true).write(true).open(&filename)
			// .and_then(|val| populate_index(val))
			.or_else(|err| 
				match err.kind() {
					ErrorKind::NotFound => {println!("file not found");log_reads_pending = false;open_new_log_file(filename)},
					_ => Err(KVError::from(err))
				}
			)?;
		let log_reader = Arc::new(Mutex::new(LogReader {
			read_buf: BufReader::new(file.try_clone()?),
			log_reads_pending: log_reads_pending
		}));
		Ok(StorageHandler{
			reader: log_reader,
			writer: BufWriter::new(file),
		})
	}

	/// Indicates whether any logs are not yet read
	/// All existing logs should be read and stored in memory before writing anything
	pub fn is_read_pending(&self) -> bool {
		//! since we are not using multiple threads yet we'll do a simple unwrap here
		self.reader.lock().unwrap().log_reads_pending
	}

	/// Get an iterator for reading log records from underlying storage file
	/// this returns a singleton logreader so make sure that the data is stored properly
	/// since consuming this iterator will lose access to all the read data
	pub fn get_log_reader(&mut self) -> Arc<Mutex<LogReader>> {
		self.reader.clone()
	}

	/// return usize here so that index map can be constructed by store
	pub fn write_record(&mut self, record: LogRecord) -> KVResult<usize> {
		if self.is_read_pending() {
			return Err(KVError::Default("Need to read existing logs before writing".to_string()));
		}
		Ok(self.writer.write(
		to_vec(&record).map_err(|err| KVError::Serialization(err))?.as_slice()
		)?)
	}
}

fn open_new_log_file(filename: String) -> KVResult<File> {
	OpenOptions::new()
				.create(true)
				.read(true)
				.write(true)
				.open(filename)
				.map_err(|err| KVError::from(err))
}
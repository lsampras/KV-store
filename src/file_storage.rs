use std::io::{BufReader, BufWriter, ErrorKind, SeekFrom};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use bson;
use bson::{de, from_document, Document, to_vec};
use crate::command::LogRecord;
use crate::error::{KVResult, KVError};

/// Log Pointer representing the position of a LogRecord in file
#[derive(Debug, Clone, Copy)]
pub struct LogPointer {
	offset: u64,
}

/// Create a struct with an enum which would handle all operations of files
/// this struct can maintain state & ownership of internal buffers
#[derive(Debug)]
pub struct StorageHandler {
	writer: BufWriter<File>,
	filename: String,
	current_offset: u64
}

impl StorageHandler {
	/// create a storage handler for reading/writing logs
	/// This method takes in a filename (path) to be used as storage
	pub fn new(filename: String) -> KVResult<Self> {
		let mut log_reads_pending = true;
		let mut file = OpenOptions::new().read(true).write(true).open(&filename)
			// .and_then(|val| populate_index(val))
			.or_else::<KVError, _>(|err| 
				match err.kind() {
					ErrorKind::NotFound => {log_reads_pending = false;Ok(open_new_log_file(&filename)?)},
					_ => Err(err)?
				}
			)?;
		let offset = file.seek(SeekFrom::End(0))?;
		Ok(StorageHandler{
			writer: BufWriter::new(file),
			filename: filename,
			current_offset: offset
		})
	}

	/// return usize here so that index map can be constructed by store
	pub fn write_record(&mut self, record: LogRecord) -> KVResult<LogPointer> {
		let write_pos = self.current_offset;
		self.current_offset += self.writer.write(
		to_vec(&record)?.as_slice()
		)? as u64;
		self.writer.flush()?;
		Ok(LogPointer{offset: write_pos})
	}

	/// read a Logrecord from underlying logs using a LogPointer 
	pub fn read_log_record_with_pointer(&self, pointer: &LogPointer) -> KVResult<LogRecord> {
		let mut file = File::open(&self.filename)?;
		file.seek(SeekFrom::Start(pointer.offset))?;
		let mut reader = BufReader::new(file);
		Ok(Document::from_reader(&mut reader)
					.and_then(|doc| Ok(from_document::<LogRecord>(doc)?))?
			)
	}
	/// read all logs
	pub fn read_all_logs(&self) -> KVResult<Vec<(LogPointer, LogRecord)>>{
		let mut records: Vec<(LogPointer, LogRecord)> = vec![];
		let mut file_offset: u64 = 0;

		let file = OpenOptions::new().read(true).open(&self.filename)?;
		let mut read_buf  = BufReader::new(file);
		loop {
			match Document::from_reader(&mut read_buf) {
				Ok(doc) =>  {
					let doc_len = to_vec(&doc)?.len() as u64;
					let record = from_document::<LogRecord>(doc).map_err(|e| KVError::Deserialization(e))?;
					records.push((LogPointer{offset:file_offset}, record));
					file_offset += doc_len;

				},
				Err(de::Error::Io(i)) => {
					if i.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(KVError::Io(i));
					}
				},
				Err(i) => return Err(KVError::from(i)),
			};
		}
		Ok(records)
	}
}

fn open_new_log_file(filename: &String) -> KVResult<File> {
	Ok(OpenOptions::new()
				.create(true)
				.read(true)
				.write(true)
				.open(filename)?)
}
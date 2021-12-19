use std::{
	io::{BufReader, BufWriter, ErrorKind, SeekFrom, prelude::*},
	fs::{File, OpenOptions},
	collections::BTreeMap,
	sync::{RwLock, Arc, Mutex}
};
use bson::{self, de, from_document, Document, to_vec};
use crate::{
	command::LogRecord,
	error::{KVResult, KVError},
	traits::Guarded,
	guard
};

/// Log Pointer representing the position of a LogRecord in file
#[derive(Debug, Clone)]
pub struct LogPointer {
	offset: u64,
	/// determines the age for logs to be used when swapping out olg logs for newer ones
	pub log_age: u8,
	filename: String
}

/// Create a struct with an enum which would handle all operations of files
/// this struct can maintain state & ownership of internal buffers
#[derive(Debug, Clone)]
pub struct StorageHandler {
	writer: Guarded<BufWriter<File>>,
	current_offset: Guarded<u64>,
	compaction_index: Guarded<u8>,
	compacted_files: Arc<RwLock<Vec<String>>>,
	temp_files: Arc<RwLock<Vec<String>>>,
}

impl StorageHandler {
	/// create a storage handler for reading/writing logs
	/// This method takes in a filename (path) to be used as storage
	pub fn new() -> KVResult<Self> {
		let filename = "foo_0.txt".to_string();
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
		let writer = BufWriter::new(file);
		Ok(StorageHandler{
			writer: guard!(writer),
			current_offset: guard!(offset),
			compaction_index: guard!(0),
			compacted_files: Arc::new(RwLock::new(vec![])),
			temp_files: Arc::new(RwLock::new(vec![filename]))
		})
	}

	fn current_wal(&self) -> KVResult<String> {
		Ok(self.temp_files.read()?.get(*self.compaction_index.lock()? as usize).unwrap().to_owned())
	}

	/// return usize here so that index map can be constructed by store
	pub fn write_record(&self, record: LogRecord) -> KVResult<LogPointer> {
		let mut write_pos = self.current_offset.lock()?;
		if let Ok(mut writer) = self.writer.lock() {
			*write_pos += writer.write(
			to_vec(&record)?.as_slice()
			)? as u64;
			writer.flush()?;
		}
		let index = *self.compaction_index.lock()?;
		Ok(LogPointer{offset: *write_pos, log_age: index, filename: self.current_wal()?})
	}

	/// read a Logrecord from underlying logs using a LogPointer 
	pub fn read_log_record_with_pointer(&self, pointer: &LogPointer) -> KVResult<LogRecord> {
		let mut file = File::open(&pointer.filename)?;
		file.seek(SeekFrom::Start(pointer.offset))?;
		let mut reader = BufReader::new(file);
		Ok(Document::from_reader(&mut reader)
					.and_then(|doc| Ok(from_document::<LogRecord>(doc)?))?
			)
	}
	/// read all logs
	/// TODO: use a hint file that contains all log pointers
	pub fn read_all_logs(&self) -> KVResult<Vec<(LogPointer, String)>>{
		self.read_all_logs_from_file(&self.current_wal()?)
	}

	fn read_all_logs_from_file(&self, filename: &String) -> KVResult<Vec<(LogPointer, String)>> {
		let mut records: Vec<(LogPointer, String)> = vec![];
		let mut file_offset: u64 = 0;

		let file = OpenOptions::new().read(true).open(filename)?;
		let mut read_buf  = BufReader::new(file);
		loop {
			match Document::from_reader(&mut read_buf) {
				Ok(doc) =>  {
					let doc_len = to_vec(&doc)?.len() as u64;
					let record = from_document::<LogRecord>(doc)?;
					records.push((LogPointer{offset:file_offset, log_age:*self.compaction_index.lock()?, filename: filename.to_owned()}, record.get_key()));
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

	fn refresh_bufwriter(&self) -> KVResult<()> {
		let new_filename = format!("foo_{}.txt", *self.compaction_index.lock()? + 1);
		self.temp_files.write()?.push(new_filename.clone());
		let new_wal_file = File::create(&new_filename)?;
		let new_writer = BufWriter::new(new_wal_file);
		let mut current_writer = self.writer.lock()?;
		*current_writer = new_writer;
		let mut compaction_index = self.compaction_index.lock()?;
		*compaction_index += 1;
		Ok(())
	}

	/// compact logs to an immutable file
	pub fn compact_logs(&self) -> KVResult<Vec<(String, LogPointer)>> {
		let mut total_log_pointers = BTreeMap::new();
		let current_wal_to_be_compacted = self.current_wal()?;
		let current_compaction_target = *self.compaction_index.lock()?;
		self.refresh_bufwriter()?;
		for (pointer, key) in self.read_all_logs_from_file(&current_wal_to_be_compacted)? {
			total_log_pointers.insert(key, pointer);
		}
		if current_compaction_target > 0 {
			let current_compacted_file = format!("compacted_{}.txt", current_compaction_target - 1);

			for (pointer, key) in self.read_all_logs_from_file(&current_compacted_file)? {
				total_log_pointers.insert(key, pointer);
			}
		}

		let mut updated_pointers = vec![];

		let compact_filename = format!("compact_{}.txt", current_compaction_target);
		let compact_out_file = File::create(&compact_filename)?;
		let mut compacted_buffer = BufWriter::new(compact_out_file);
		let mut write_pos = 0;
	
		for i in total_log_pointers.keys().cloned() {
			let record = self.read_log_record_with_pointer(total_log_pointers.get(&i).unwrap())?;
			let incr = compacted_buffer.write(
			to_vec(&record)?.as_slice()
			)? as u64;
			let pointer = LogPointer{offset: write_pos, log_age: current_compaction_target + 1, filename: compact_filename.clone()};
			write_pos += incr;
			updated_pointers.push((i, pointer));
		}
		self.compacted_files.write()?.push(compact_filename);
		Ok(updated_pointers)
	}
}

fn open_new_log_file(filename: &String) -> KVResult<File> {
	Ok(File::create(filename)?)
}
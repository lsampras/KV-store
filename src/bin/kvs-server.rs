use clap::{crate_authors, crate_description, crate_version};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use kvs::{KvStore, error::KVResult, logging::{create_logger}, traits::KvsEngine, threadpool::NaiveThreadPool};
use std::io::{Read, Write, BufReader, Cursor};
use std::net::{TcpListener, Shutdown};
use std::process::exit;
use bson::{Document, from_document};
use std::str;
use std::sync::Arc;

#[macro_use]
extern crate slog;
extern crate sled;

use sled::Db;
#[derive(StructOpt)]
#[structopt(
    name = "Key-Value Server",
	about = crate_description!(),
    author = crate_authors!()
)]
pub struct Opt {
	#[structopt(long = "addr", help = "Set server address", default_value = "127.0.0.1:4000")]
    addr: String,
	#[structopt(long = "engine", help = "Set KV engine", default_value = "kvs")]
    engine: String,
}


#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(name = "command")]
pub enum Command {
	#[structopt(
		name = "set",
		help = "Set value for a given key in kv store"
	)]
	Set {
        key: String,
        value: String,
	},
	#[structopt(
		name = "get",
		help = "Get value for a given key in kv store"
	)]
	Get{
        key: String
	},
	#[structopt(
		name = "rm",
		help = "Delete a given key from kv store"
	)]
	Delete{
        key: String,
	},
}

pub fn from_bytes(bytes: &mut [u8]) -> KVResult<Command> {
	let mut reader = Cursor::new(bytes);
	Ok(from_document::<Command>(Document::from_reader(&mut reader)?)?)
}

fn run_kv_command<T>(store: &Box<T>, command: Command) -> KVResult<Option<String>>
	where T: KvsEngine,
{
	// let mut store = KvStore::new()?;
	Ok(match command {
		Command::Set{key, value} => {store.set(key, value)?;None},
		Command::Get{key} => Some(store.get(key)?.unwrap_or("No key found".to_string())),
		Command::Delete{key} => {store.remove(key)?; None},
	})
}

#[derive(Debug, Clone)]
struct Sled {
	db: Box<Db>
}

impl KvsEngine for Sled {

	/// Get the value of a key in a KvStore
	fn get(&self, key: String) -> KVResult<Option<String>> {
		Ok(
			self.db.get(key).unwrap()
				.and_then(|i| Some(str::from_utf8(i.as_ref()).unwrap().to_owned())))
	}

	/// Add or update the value of an existing key
	fn set(&self, key: String, value: String) -> KVResult<()> {
		self.db.insert(key.as_bytes(), value.as_bytes()).unwrap();
		Ok(())
	}

	/// clear a given key from kv store
	fn remove(&self, key: String) -> KVResult<()> {
		self.db.remove(key.as_bytes()).unwrap();
		Ok(())
	}
}


#[allow(unused)]
fn main() -> KVResult<()>{
	let opt = Opt::from_args();
	println!("{:?}, {:?}", opt.addr, opt.engine);
	let url = opt.addr;
	let engine = Arc::new(opt.engine);

	let logger = Arc::new(create_logger());
	info!(logger,
		"Starting KVS Server version {version}\n with address {address} and engine {storage}",
		version=crate_version!(), address=&url, storage=&engine
	);
	let listener = TcpListener::bind(url)?;
	let mut threads = NaiveThreadPool::new(6);
	threads.initialize_pool();
	for stream in listener.incoming() {
		let logger2 = logger.clone();
		let engine2 = engine.clone();
		threads.spawn(move || {
			let mut buffer = vec![];
			let mut tcp_stream = stream.unwrap();
			let mut reader = BufReader::new(&tcp_stream);
			reader.read_to_end(&mut buffer).unwrap();
			info!(logger2, "Bytes receievd: {:?}", &buffer);
			tcp_stream.shutdown(Shutdown::Read).unwrap();
			match engine2.as_str() {
				"kvs" => {
					let store = Box::new(KvStore::new().unwrap());
	
					match run_kv_command(&store, from_bytes(&mut buffer).unwrap()).unwrap() {
						Some(str) => {
							info!(logger2, "writing {:?}", str.as_bytes());
							tcp_stream.write(str.as_bytes()).unwrap();
							tcp_stream.flush().unwrap();
						},
						_ => {}
					};
				},
				"sled" => {
					let store = Box::new(Sled {
						db: Box::new(sled::open("sled.txt").unwrap())
					});
	
					match run_kv_command(&store, from_bytes(&mut buffer).unwrap()).unwrap() {
						Some(str) => {
							info!(logger2, "writing {:?}", str.as_bytes());
							tcp_stream.write(str.as_bytes()).unwrap();
							tcp_stream.flush().unwrap();
						},
						_ => {}
					};
				},
				_ => {println!("Storage Engine not supported");exit(1);}
			};
			tcp_stream.shutdown(Shutdown::Write).unwrap();
		});
    }
	Ok(())
}
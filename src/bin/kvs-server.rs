use clap::{crate_authors, crate_description, crate_version};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use kvs::{error::KVResult, logging::{create_logger}, KvStore};
use std::io::{Read, Write, BufReader, Cursor};
use std::net::{TcpListener, Shutdown};
use bson::{Document, from_document};

#[macro_use]
extern crate slog;
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

fn run_kv_command(store: &mut KvStore, command: Command) -> KVResult<Option<String>> {
	// let mut store = KvStore::new()?;
	Ok(match command {
		Command::Set{key, value} => {store.set(key, value)?;None},
		Command::Get{key} => Some(store.get(key)?.unwrap_or("No key found".to_string())),
		Command::Delete{key} => {store.remove(key)?; None},
	})
}

#[allow(unused)]
fn main() -> KVResult<()>{
	let opt = Opt::from_args();
	println!("{:?}, {:?}", opt.addr, opt.engine);
	let url = opt.addr;
	let engine = opt.engine;
	let logger = create_logger();
	info!(logger,
		"Starting KVS Server version {version}\n with address {address} and engine {storage}",
		version=crate_version!(), address=&url, storage=&engine
	);
	let mut store = KvStore::new()?;
	let listener = TcpListener::bind(url)?;
	for stream in listener.incoming() {
		let mut buffer = vec![];
		let mut tcp_stream = stream?;
        let mut reader = BufReader::new(&tcp_stream);
		reader.read_to_end(&mut buffer)?;
		info!(logger, "Bytes receievd: {:?}", &buffer);
		tcp_stream.shutdown(Shutdown::Read)?;
		match run_kv_command(&mut store, from_bytes(&mut buffer)?)? {
			Some(str) => {
				info!(logger, "writing {:?}", str.as_bytes());
				tcp_stream.write(str.as_bytes())?;
				tcp_stream.flush()?;
			},
			_ => {}
		};
		tcp_stream.shutdown(Shutdown::Write)?;
    }
	Ok(())
}
use clap::{crate_authors, crate_description};
use structopt::StructOpt;
use std::net::{TcpStream, Shutdown};
use std::{process::exit, str};
use kvs::{KvStore, error::KVResult};
use std::io::{Read, Write, Cursor};
use bson::{to_vec, from_document, Document};
use serde::{Serialize, Deserialize};

#[derive(StructOpt)]
#[structopt(
    name = "Key-Value Client",
	about = crate_description!(),
    author = crate_authors!()
)]
pub struct Opt {
	#[structopt(subcommand)]
    pub cmd: Option<Command>,
	#[structopt(long = "addr", help = "Set server address", default_value = "127.0.0.1:4000")]
    server: String,
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

#[allow(unused)]
fn main() -> KVResult<()> {
	let opt = Opt::from_args();
	let server_address = opt.server;
	match opt.cmd {
		Some(command) => {
			let serialized = to_vec(&command)?;
			let mut stream = TcpStream::connect(server_address)?;
			stream.write(serialized.as_slice())?;
			stream.flush()?;
			stream.shutdown(Shutdown::Write)?;
			match command {
				Command::Get{..} => {
					let mut res = vec![];
					stream.read_to_end(&mut res)?;
					println!("bytes read :{}", str::from_utf8(&res).unwrap());
				},
				_ => {}
			}
			Ok(())
		},
		None => {
			eprintln!("Specify a param");
			exit(1);
		},
	}
}
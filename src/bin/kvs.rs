use clap::{crate_authors, crate_description};
use structopt::StructOpt;
use std::process::exit;
use kvs::{KvStore, error::KVResult};
#[derive(StructOpt)]
#[structopt(
    name = "Key-Value Server",
	about = crate_description!(),
    author = crate_authors!()
)]
pub struct Opt {
	#[structopt(subcommand)]
    pub cmd: Option<Command>,
}


#[derive(StructOpt, Debug)]
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

fn run_kv_command(command: Command) -> KVResult<()> {
	let mut store = KvStore::new()?;
	Ok(match command {
		Command::Set{key, value} => store.set(key, value)?,
		Command::Get{key} => println!("{}", store.get(key)?),
		Command::Delete{key} => store.remove(key)?,
	})
}

#[allow(unused)]
fn main() {
	let opt = Opt::from_args();
	// println!("{:?}, {:?}", opt.data, opt.cmd.unwrap());
	match opt.cmd {
		Some(command) => {
			match run_kv_command(command) {
				Err(e) => {println!("Error occured: {:?}", e); exit(1);},
				Ok(_) => {},
			}
		},
		None => {
			eprintln!("Specify a param");
			exit(1);
		},
	};
}
use clap::{crate_authors, crate_description};
use structopt::StructOpt;


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

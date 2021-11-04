use clap::{crate_authors, crate_description, crate_version};
use structopt::StructOpt;
use kvs::logging::{create_logger};

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


#[allow(unused)]
fn main() {
	let opt = Opt::from_args();
	println!("{:?}, {:?}", opt.addr, opt.engine);
	let url = opt.addr;
	let engine = opt.engine;
	let logger = create_logger();
	info!(logger,
		"Starting KVS Server version {version}\n with address {address} and engine {storage}",
		version=crate_version!(), address=&url, storage=&engine
	);
}
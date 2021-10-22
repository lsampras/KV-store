use kvs::cli::{Opt, Command};
use std::process::exit;
use structopt::StructOpt;

#[allow(unused)]
fn main() {
	let opt = Opt::from_args();
	// println!("{:?}, {:?}", opt.data, opt.cmd.unwrap());
	match opt.cmd {
		Some(Command::Set{key, value}) => {
			unimplemented!();
		},
		Some(Command::Get{key}) => {
			unimplemented!();
		},
		Some(Command::Delete{key}) => {
			unimplemented!();
		},
		None => {
			eprintln!("Specify a param");
			exit(1);
		},
	}
}
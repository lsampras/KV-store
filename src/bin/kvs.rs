use kvs::cli::Opt;
use structopt::StructOpt;

fn main() {
	let opt = Opt::from_args();
	println!("{:?}, {:?}", opt.data, opt.cmd.unwrap());
}
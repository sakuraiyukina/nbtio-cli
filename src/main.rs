use clap::{arg, Command};
use std::string::String;

mod readnbt;
mod readleveldb;

fn cli() {
	let cli_flags = Command::new("")
		.disable_help_flag(true)
		.args(&[
			arg!(-i --input <FILE> "read form standard input"),
			arg!(-c --stdout ... "write to standard output"),
			arg!(-h --help "show this help message and exit.")
				.required(false)
		])
		.get_matches();


	if let Some(cli_input) = cli_flags.get_one::<String>("input") {
		let input_path = cli_input;
		println!("Input file: {}", input_path);
		readleveldb::print_db(input_path);
	}
	else {
		eprint!("The value cannot be null.");
	}

}
fn main() {
	cli();
}
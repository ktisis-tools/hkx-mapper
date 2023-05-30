// Dependencies

mod path;
mod reader;
mod scanner;

use scanner::ExeScanner;

use clap::{Arg, Command};
use simple_error::SimpleError;

// Constant values

const SIG_HKCLASS: &str = r"8B 44 24 30 89 41 14 48 8B 44 24";

// Main program

fn main() {
	let args = Command::new("hkx-mapper").arg(
		Arg::new("path")
			.long("path")
			.short('p')
			.help("Path to the game's base install.")
			.required(true)
	).get_matches();

	let path_str = args.get_one::<String>("path").expect("Failed to parse game path!");
	println!("{}", match scan(path_str) {
		Ok(_) => "Success".to_owned(),
		Err(err) => format!("Error: {err}")
	});
}

fn scan(path_str: &str) -> Result<(), SimpleError> {
	let scanner = ExeScanner::new(path_str)?;

	let header = scanner.get_header().clone();

	let mut code = scanner.section(".text")?;
	if let Some(ptr) = code.find_signature(SIG_HKCLASS) {
		println!("hkClass signature found\n\
			- File offset: {:X}\n\
			- Virtual offset: {:X}",
			ptr.file_offset(),
			ptr.virtual_offset() + header.image_base as usize
		)

		// TODO: xref scanning

	} else {
		return Err(SimpleError::new("Failed to find signature."));
	}

	code.clear();

	Ok(())
}
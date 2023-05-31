// Dependencies

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
	let start = std::time::Instant::now();

	let scanner = ExeScanner::new(path_str)?;

	let header = scanner.get_header().clone();

	let code = scanner.section(".text")?;
	if let Some(func) = code.find_func_signature(SIG_HKCLASS) {
		let ptr = &func.pointer;
		println!("hkClass signature found\n\
			- File offset: {:X}\n\
			- Virtual offset: {:X}",
			ptr.file_offset(),
			ptr.virtual_offset() + header.image_base as usize
		);

		// TODO: xref scanning, read args for class information

		let xrefs = code.find_func_xrefs(&func);
		println!("Found {} xref(s)", xrefs.len());

	} else {
		return Err(SimpleError::new("Failed to find signature."));
	}

	let elapsed = start.elapsed();
	println!("Elapsed: {:?}", elapsed);

	Ok(())
}
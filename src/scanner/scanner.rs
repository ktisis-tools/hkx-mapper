// Dependencies

use crate::{
	scanner::SectionScanner,
	reader::{ExeReader, ExeHeader}
};
use super::path::GameDirectory;

use simple_error::SimpleError;

// Scanner

pub struct ExeScanner {
	reader: ExeReader
}

impl ExeScanner {
	pub fn new(path_str: &str) -> Result<Self, SimpleError> {
		let game_dir = GameDirectory::at(path_str)?;
		let exe_path = game_dir.get_exe_path();
		let reader = ExeReader::open(exe_path.as_path())?;
		Ok(ExeScanner { reader })
	}

	pub fn get_header(&self) -> &ExeHeader { &self.reader.header }

	pub fn section(mut self, name: &str) -> Result<SectionScanner, SimpleError> {
		SectionScanner::new(name, &mut self.reader)
	}
}
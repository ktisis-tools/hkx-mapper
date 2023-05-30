// Dependencies

use crate::reader::{ExeReader, SectionHeader};
use sigscanner::{
	signatures::parse_sig_str,
	scanning::find_sig
};

use simple_error::SimpleError;

// Section

pub struct SectionScanner {
	pub name: String,
	pub header: SectionHeader,
	data: Vec<u8>
}

impl SectionScanner {
	pub fn new(name: &str, reader: &mut ExeReader) -> Result<Self, SimpleError> {
		let header = reader.get_section_header(name)
			.ok_or(SimpleError::new(format!("Failed to find header for section '{name}'")))?
			.clone();
		let data = reader.read_section_data(&header)?;
		Ok(SectionScanner {
			name: name.to_owned(),
			header,
			data
		})
	}

	pub fn find_signature(&self, sig_str: &str) -> Option<Pointer> {
		let sig = parse_sig_str(sig_str);
		let data_ptr = self.data.as_ptr();

		let offset = unsafe {
			let result = match find_sig(data_ptr, self.header.raw_data_size as usize, sig) {
				v if v != 0 as _ => Some(v),
				_ => None
			}?;
			result.offset_from(data_ptr) as usize
		};

		let pointer = Pointer::new(offset, &self.header);
		Some(pointer)
	}

	pub fn clear(&mut self) {
		self.data.clear();
	}
}

// Pointer

#[derive(Debug)]
pub struct Pointer {
	value: usize,
	raw_offset: u32,
	virtual_offset: u32
}

impl Pointer {
	pub fn new(value: usize, header: &SectionHeader) -> Self { Pointer {
		value,
		raw_offset: header.raw_data_ptr,
		virtual_offset: header.virtual_addr
	} }

	pub fn raw_value(&self) -> usize { self.value }
	pub fn file_offset(&self) -> usize { self.value + self.raw_offset as usize }
	pub fn virtual_offset(&self) -> usize { self.value + self.virtual_offset as usize }
}
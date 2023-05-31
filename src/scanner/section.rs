// Dependencies

use crate::reader::{ExeReader, SectionHeader};
use super::asm::{Pointer, Function};

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

	pub fn find_func_signature(&self, sig_str: &str) -> Option<Function> {
		let pointer = self.find_signature(sig_str)?;
		Some(Function::new(pointer))
	}

	pub fn find_func_xrefs(&self, func: &Function) -> Vec<Pointer> {
		let mut results = Vec::<Pointer>::new();

		let data_ptr = self.data.as_ptr();
		let data_end = unsafe { data_ptr.add(self.header.raw_data_size as usize) };

		let mut cursor = data_ptr;
		unsafe {
			let func_ptr = data_ptr.add(func.pointer.raw_value());
			while cursor < data_end {
				let mut advance = 1;
				if *cursor == 0xE8 {
					let ptr_call = *(cursor.add(1) as *const u32);
					let ptr_needle = func_ptr.offset_from(cursor.add(5)) as u32;
					if ptr_call == ptr_needle {
						let pointer = Pointer::new(cursor.offset_from(data_ptr) as usize, &self.header);
						results.push(pointer);
					}
					advance += 4;
				}
				cursor = cursor.add(advance);
			}
		}

		return results;
	}
}
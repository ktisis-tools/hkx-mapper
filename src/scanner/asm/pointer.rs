// Dependencies

use crate::reader::SectionHeader;

// Pointer

#[derive(Debug, Copy, Clone)]
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
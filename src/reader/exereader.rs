// Dependencies

use crate::reader::Reader;

use std::path::Path;

use simple_error::SimpleError;

// ExeReader

pub struct ExeReader {
	reader: Reader,

	pub header: ExeHeader
}

impl ExeReader {
	pub fn open(path: &Path) -> Result<Self, SimpleError> {
		let reader = Reader::open(path)?;
		let this = ExeReader {
			reader,
			header: Default::default()
		};
		this.parse_header()
	}

	fn parse_header(mut self) -> Result<Self, SimpleError> {
		self.header = Default::default();

		// DOS header
		let pe_offset = self.reader.read_at::<u32>(0x3C) as u64;

		// PE header
		let section_ct: u16 = self.reader.read_at(pe_offset + 0x06);
		let oh_size = self.reader.read_at::<u16>(pe_offset + 0x14) as u64;

		// Optional header
		let oh_offset = pe_offset + 0x18;
		let image_base: u64 = self.reader.read_at(oh_offset + 0x18);
		self.header.image_base = image_base;

		// Section table
		let st_offset = oh_offset + oh_size;
		for i in 0..section_ct {
			let offset = st_offset + i as u64 * 0x28;
			let section = SectionHeader {
				index: i,
				name: self.reader.read_str_at(offset, 8)?,
				virtual_size: self.reader.read_at(offset + 0x08),
				virtual_addr: self.reader.read_at(offset + 0x0C),
				raw_data_size: self.reader.read_at(offset + 0x10),
				raw_data_ptr: self.reader.read_at(offset + 0x14)
			};
			self.header.sections.push(section);
		}

		Ok(self)
	}

	pub fn get_section_header(&self, name: &str) -> Option<&SectionHeader> {
		self.header.get_section(name)
	}

	pub fn read_section_data(&mut self, section: &SectionHeader) -> Result<Vec<u8>, SimpleError> {
		let mut data = vec![0u8; section.raw_data_size as usize];
		self.reader.seek_to(section.raw_data_ptr as u64)?;
		self.reader.read_raw(&mut data)?;
		Ok(data)
	}
}

// Header

#[derive(Clone, Debug, Default)]
pub struct ExeHeader {
	pub image_base: u64,
	pub sections: Vec<SectionHeader>
}

impl ExeHeader {
	pub fn get_section(&self, name: &str) -> Option<&SectionHeader> {
		self.sections.iter().find(|x| x.name == name)
	}
}

// Section

#[derive(Debug, Clone)]
pub struct SectionHeader {
	pub index: u16,
	pub name: String,
	pub virtual_size: u32,
	pub virtual_addr: u32,
	pub raw_data_size: u32,
	pub raw_data_ptr: u32
}
// Dependencies

use std::{
	fs::File,
	io::{Read, Seek, SeekFrom},
	path::Path,
	ptr::read,
	mem::size_of
};

use simple_error::{SimpleError, SimpleResult};

// Constant values

const BUFFER_SIZE: usize = 256;

// Reader

pub struct Reader {
	buffer: Vec<u8>,
	file: File
}

impl Reader {
	pub fn new(file: File) -> Self {
		let buffer = Vec::<u8>::with_capacity(BUFFER_SIZE);
		Reader { buffer, file }
	}

	pub fn open(path: &Path) -> Result<Self, SimpleError> {
		match File::open(path) {
			Ok(file) => Ok(Reader::new(file)),
			Err(err) => Err(SimpleError::from(err))
		}
	}

	pub fn seek(&mut self, from: SeekFrom) -> SimpleResult<u64> {
		match self.file.seek(from) {
			Ok(val) => Ok(val),
			Err(err) => Err(SimpleError::from(err))
		}
	}

	pub fn seek_to(&mut self, n: u64) -> SimpleResult<u64> {
		self.seek(SeekFrom::Start(n))
	}

	pub fn read<T>(&mut self) -> T {
		// this *should* check the buffer size but I like speedy code <3
		let size = size_of::<T>();
		unsafe {
			self.buffer.set_len(size);
			self.file.read_exact(&mut self.buffer).expect("Read failed");
			read(self.buffer.as_ptr() as *const T)
		}
	}

	pub fn read_at<T>(&mut self, offset: u64) -> T {
		self.seek_to(offset).expect("Seek failed");
		self.read()
	}

	pub fn read_str(&mut self, len: usize) -> Result<String, SimpleError> {
		let mut buffer = vec![0u8; len];
		self.file.read_exact(&mut buffer).expect("Read failed");

		// Truncate to before null terminator.
		if let Some(term) = buffer.iter().position(|x| x == &0) {
			buffer.truncate(term);
		}

		match std::str::from_utf8(&buffer) {
			Ok(val) => Ok(val.to_owned()),
			Err(err) => Err(SimpleError::from(err))
		}
	}

	pub fn read_str_at(&mut self, offset: u64, len: usize) -> Result<String, SimpleError> {
		match self.seek_to(offset) {
			Ok(_) => self.read_str(len),
			Err(err) => Err(SimpleError::from(err))
		}
	}

	pub fn read_raw(&mut self, buffer: &mut [u8]) -> Result<(), SimpleError> {
		match self.file.read_exact(buffer) {
			Ok(_) => Ok(()),
			Err(err) => Err(SimpleError::from(err))
		}
	}
}
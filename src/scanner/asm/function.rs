// Dependencies

use crate::scanner::asm::Pointer;

use std::ptr;

// Function

pub struct Function {
	pub pointer: Pointer,
	pub bytes: Vec<u8>
}

impl Function {
	pub fn new(pointer: Pointer, bytes: Vec<u8>) -> Self {
		Function { pointer, bytes }
	}

	pub unsafe fn get_bytes(tar_ptr: *const u8, needle_offset: &mut isize) -> Vec<u8> {
		// Find start and end of function.
		let mut func_start: Option<*const u8> = None;
		let mut func_end: Option<*const u8> = None;
		for i in 0.. {
			let mut has_none = false;
			for (option, offset) in [(&mut func_start, -i), (&mut func_end, i)] {
				if option.is_some() { continue };
				let ptr = tar_ptr.offset(offset);
				let next = *ptr.offset(offset.signum());
				if next == 0xCC || (offset < 0 && next == 0xC3) {
					*option = Some(ptr);
				} else {
					has_none |= true;
				}
			}
			if !has_none { break };
		};

		let start = func_start.unwrap();
		let size = func_end.unwrap().offset_from(start) as usize;
		*needle_offset = tar_ptr.offset_from(start);

		let mut func_bytes = Vec::with_capacity(size);
		ptr::copy(start, func_bytes.as_mut_ptr(), size);
		func_bytes.set_len(size);

		func_bytes
	}
}
// Dependencies

use crate::scanner::asm::Pointer;

// Function

#[derive(Debug)]
pub struct Function {
	pub pointer: Pointer
}

impl Function {
	pub fn new(pointer: Pointer) -> Self {
		Function { pointer }
	}
}
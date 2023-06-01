// Dependencies

use super::{Pointer, Function};
use crate::scanner::SectionScanner;

use std::{
	ops::Add,
	collections::HashMap
};

use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic, OpKind, Register};

// Call

pub struct Call {
	pub pointer: Pointer,
	pub calling_func: Function,
	pub call_pos: isize
}

impl Call {
	pub fn from_xref(scanner: &SectionScanner, pointer: Pointer) -> Self {
		let mut call_pos = 0;
		let calling_func = scanner.get_func_at(pointer, &mut call_pos);
		Call { pointer, calling_func, call_pos }
	}

	pub fn get_call_values(&self) -> CallValues {
		let mut state = CallValues::new();

		let bytes = &self.calling_func.bytes[0..self.call_pos as usize];
		let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);

		/*
			We make 2 assumptions here:
			1. MOV and LEA are the only instructions used to set parameters
			2. All immediate values are declared locally (ie. not passed into the calling function).
			This will probably break down if used for anything other than hkClass constructors.
		*/

		for op in decoder.iter() {
			match op.mnemonic() {
				Mnemonic::Mov | Mnemonic::Lea => {
					state.set_val_op(&op);
				},
				Mnemonic::Xor => {
					let reg0 = op.op0_register();
					let reg1 = op.op1_register();
					// safe to assume this is 0
					if reg0 == reg1 {
						state.values.insert(OpDst::Reg(reg0), OpVal::Immediate(0));
					}
				},
				_ => continue
			}
		}

		state
	}
}

// CallValues

#[derive(Default)]
pub struct CallValues {
	pub values: HashMap<OpDst, OpVal>
}

impl CallValues {
	pub fn new() -> Self { CallValues { ..Default::default() } }

	pub fn get_args(&self, ct: usize) -> Vec<OpVal> {
		let mut args: Vec<OpVal> = vec![OpVal::Immediate(0); ct];
		let max = ct as u64 * 8;
		for (dst, val) in &self.values {
			let i = match dst {
				OpDst::Reg(Register::RCX) => 0,
				OpDst::Reg(Register::RDX) => 1,
				OpDst::Reg(Register::R8) => 2,
				OpDst::Reg(Register::R9D) => 3,
				OpDst::Stack(pos) if *pos > 24 && *pos < max => {
					*pos as usize / 8
				},
				_ => continue
			};
			args[i] = *val;
		}
		args
	}

	pub fn set_val_op(&mut self, op: &Instruction) {
		let dst = match op.op0_kind() {
			OpKind::Register => OpDst::Reg(op.op0_register()),
			OpKind::Memory => OpDst::Stack(op.memory_displacement64()),
			err => panic!("Encountered unknown OpKind: {err:?}")
		};

		let val = match op.op1_kind() {
			OpKind::Immediate32 => {
				OpVal::Immediate(op.immediate(1))
			},
			OpKind::Memory => { // LEA
				if op.is_ip_rel_memory_operand() {
					OpVal::Relative(op.ip_rel_memory_address())
				} else {
					let val = *self.get_val_reg(op.op1_register()).unwrap_or(&OpVal::Immediate(0));
					val + op.memory_displacement64()
				}
			},
			OpKind::Register => { // MOV
				*self.get_val_reg(op.op1_register()).unwrap_or(&OpVal::Immediate(0))
			},
			_ => OpVal::Unknown
		};

		self.values.insert(dst, val);
	}

	pub fn get_val_reg(&self, reg: Register) -> Option<&OpVal> {
		let dst = OpDst::Reg(reg);
		self.values.get(&dst)
	}
}

// OpDst & OpVal

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum OpDst {
	Reg(Register),
	Stack(u64)
}

#[derive(Debug, Copy, Clone)]
pub enum OpVal {
	Unknown,
	Immediate(u64),
	Relative(u64)
}

impl Add<u64> for OpVal {
	type Output = Self;

	fn add(self, other: u64) -> Self {
		match self {
			Self::Unknown => Self::Unknown,
			Self::Immediate(o) => Self::Immediate(o + other),
			Self::Relative(o) => Self::Relative(o + other)
		}
	}
}
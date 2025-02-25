// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::formatter::fmt_consts::*;
use crate::formatter::nasm::FormatterString;
use crate::iced_constants::IcedConstants;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use lazy_static::lazy_static;

pub(super) struct Info {
	pub(super) keyword: &'static FormatterString,
	pub(super) bcst_to: &'static FormatterString,
}

// GENERATOR-BEGIN: BcstTo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static BCST_TO_DATA: [u8; 50] = [
	0x01,
	0x01,
	0x01,
	0x02,
	0x01,
	0x03,
	0x03,
	0x02,
	0x02,
	0x01,
	0x01,
	0x01,
	0x03,
	0x02,
	0x01,
	0x02,
	0x01,
	0x01,
	0x02,
	0x02,
	0x04,
	0x04,
	0x03,
	0x03,
	0x02,
	0x02,
	0x02,
	0x04,
	0x03,
	0x02,
	0x03,
	0x02,
	0x02,
	0x03,
	0x03,
	0x05,
	0x05,
	0x04,
	0x04,
	0x03,
	0x03,
	0x03,
	0x05,
	0x04,
	0x03,
	0x04,
	0x04,
	0x03,
	0x03,
	0x04,
];
// GENERATOR-END: BcstTo

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static MEM_SIZE_TBL_DATA: [u8; 160] = [
	0x00,
	0x01,
	0x0B,
	0x02,
	0x09,
	0x09,
	0x08,
	0x0C,
	0x0D,
	0x01,
	0x0B,
	0x02,
	0x09,
	0x08,
	0x0C,
	0x0D,
	0x03,
	0x03,
	0x03,
	0x0B,
	0x02,
	0x09,
	0x00,
	0x00,
	0x09,
	0x08,
	0x00,
	0x00,
	0x0B,
	0x02,
	0x09,
	0x0A,
	0x08,
	0x0B,
	0x04,
	0x05,
	0x07,
	0x06,
	0x00,
	0x00,
	0x00,
	0x00,
	0x0A,
	0x0D,
	0x00,
	0x0A,
	0x0E,
	0x0D,
	0x0B,
	0x0B,
	0x02,
	0x02,
	0x02,
	0x02,
	0x02,
	0x02,
	0x09,
	0x09,
	0x09,
	0x09,
	0x09,
	0x09,
	0x09,
	0x09,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x08,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0D,
	0x0B,
	0x02,
	0x02,
	0x0B,
	0x02,
	0x0B,
	0x0B,
	0x02,
	0x02,
	0x09,
	0x09,
	0x09,
	0x0B,
	0x02,
	0x09,
	0x02,
	0x09,
	0x09,
	0x02,
	0x02,
	0x0B,
	0x0B,
	0x02,
	0x02,
	0x09,
	0x09,
	0x09,
	0x0B,
	0x02,
	0x09,
	0x02,
	0x09,
	0x09,
	0x02,
	0x02,
	0x0B,
	0x0B,
	0x02,
	0x02,
	0x09,
	0x09,
	0x09,
	0x0B,
	0x02,
	0x09,
	0x02,
	0x02,
	0x09,
	0x09,
	0x02,
];
// GENERATOR-END: MemorySizes

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Box<[Info; IcedConstants::MEMORY_SIZE_ENUM_COUNT]> = {
		let mut v = Vec::with_capacity(IcedConstants::MEMORY_SIZE_ENUM_COUNT);
		let c = &*FORMATTER_CONSTANTS;
		for (i, &mem_keywords) in MEM_SIZE_TBL_DATA.iter().enumerate() {
			let keyword = match mem_keywords {
				// GENERATOR-BEGIN: MemoryKeywordsMatch
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				0x00 => &c.empty,
				0x01 => &c.byte,
				0x02 => &c.dword,
				0x03 => &c.far,
				0x04 => &c.fpuenv14,
				0x05 => &c.fpuenv28,
				0x06 => &c.fpustate108,
				0x07 => &c.fpustate94,
				0x08 => &c.oword,
				0x09 => &c.qword,
				0x0A => &c.tword,
				0x0B => &c.word,
				0x0C => &c.yword,
				0x0D => &c.zword,
				0x0E => &c.mem384,
				// GENERATOR-END: MemoryKeywordsMatch
				_ => unreachable!(),
			};
			let bcst_to = if i < IcedConstants::FIRST_BROADCAST_MEMORY_SIZE as usize {
				&c.empty
			} else {
				match BCST_TO_DATA[i - IcedConstants::FIRST_BROADCAST_MEMORY_SIZE as usize] {
					// GENERATOR-BEGIN: BroadcastToKindMatch
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					0x00 => &c.empty,
					0x01 => &c.b1to2,
					0x02 => &c.b1to4,
					0x03 => &c.b1to8,
					0x04 => &c.b1to16,
					0x05 => &c.b1to32,
					// GENERATOR-END: BroadcastToKindMatch
					_ => unreachable!(),
				}
			};

			v.push(Info { keyword, bcst_to });
		}
		#[allow(clippy::unwrap_used)]
		v.into_boxed_slice().try_into().ok().unwrap()
	};
}

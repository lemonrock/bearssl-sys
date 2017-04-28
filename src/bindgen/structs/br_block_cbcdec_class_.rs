// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_block_cbcdec_class_
{
	pub context_size: usize,
	pub block_size: c_uint,
	pub log_block_size: c_uint,
	pub init: Option<unsafe extern "C" fn(ctx: *mut *const br_block_cbcdec_class, key: *const c_void, key_len: usize)>,
	pub run: Option<unsafe extern "C" fn(ctx: *const *const br_block_cbcdec_class, iv: *mut c_void, data: *mut c_void, len: usize)>,
}

impl Clone for br_block_cbcdec_class_
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_block_cbcdec_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

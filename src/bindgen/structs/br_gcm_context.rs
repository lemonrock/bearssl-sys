// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_gcm_context
{
	pub vtable: *const br_aead_class,
	pub bctx: *mut *const br_block_ctr_class,
	pub gh: br_ghash,
	pub h: [c_uchar; 16usize],
	pub j0_1: [c_uchar; 12usize],
	pub buf: [c_uchar; 16usize],
	pub y: [c_uchar; 16usize],
	pub j0_2: u32,
	pub jc: u32,
	pub count_aad: u64,
	pub count_ctr: u64,
}

impl Clone for br_gcm_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_gcm_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

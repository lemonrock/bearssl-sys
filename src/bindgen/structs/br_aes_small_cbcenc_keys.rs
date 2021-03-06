// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub struct br_aes_small_cbcenc_keys
{
	pub vtable: *const br_block_cbcenc_class,
	pub skey: [u32; 60usize],
	pub num_rounds: c_uint,
}

impl Default for br_aes_small_cbcenc_keys
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

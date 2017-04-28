// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub struct br_hmac_drbg_context
{
	pub vtable: *const br_prng_class,
	pub K: [c_uchar; 64usize],
	pub V: [c_uchar; 64usize],
	pub digest_class: *const br_hash_class,
}

impl Default for br_hmac_drbg_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

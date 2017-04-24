// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[allow(missing_debug_implementations)]
#[allow(missing_copy_implementations)]
pub struct br_sslrec_in_cbc_context
{
	pub vtable: *const br_sslrec_in_cbc_class,
	pub seq: uint64_t,
	pub bc: br_sslrec_in_cbc_context_AnonymousUnion_bc,
	pub mac: br_hmac_key_context,
	pub mac_len: size_t,
	pub iv: [c_uchar; 16usize],
	pub explicit_IV: c_int,
}

impl Default for br_sslrec_in_cbc_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

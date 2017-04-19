// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_aes_x86ni_cbcdec_keys
{
	pub vtable: *const br_block_cbcdec_class,
	pub skey: br_aes_x86ni_cbcdec_keys_AnonymousUnion_skey,
	pub num_rounds: c_uint,
}

impl Clone for br_aes_x86ni_cbcdec_keys
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_aes_x86ni_cbcdec_keys
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

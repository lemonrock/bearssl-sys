// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_engine_context_AnonymousStruct_cpu
{
	pub dp: *mut uint32_t,
	pub rp: *mut uint32_t,
	pub ip: *const c_uchar,
}

impl Default for br_ssl_engine_context_AnonymousStruct_cpu
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
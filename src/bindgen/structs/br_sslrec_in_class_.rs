// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_sslrec_in_class_
{
	pub context_size: size_t,
	pub check_length: Option<unsafe extern "C" fn(ctx: *const *const br_sslrec_in_class, record_len: size_t) -> c_int>,
	pub decrypt: Option<unsafe extern "C" fn(ctx: *mut *const br_sslrec_in_class, record_type: c_int, version: c_uint, payload: *mut c_void, len: *mut size_t) -> *mut c_uchar>,
}

impl Default for br_sslrec_in_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

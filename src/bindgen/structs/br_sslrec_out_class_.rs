// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_sslrec_out_class_
{
	pub context_size: usize,
	pub max_plaintext: Option<unsafe extern "C" fn(ctx: *const *const br_sslrec_out_class, start: *mut usize, end: *mut usize)>,
	pub encrypt: Option<unsafe extern "C" fn(ctx: *mut *const br_sslrec_out_class, record_type: c_int, version: c_uint, plaintext: *mut c_void, len: *mut usize) -> *mut c_uchar>,
}

impl Clone for br_sslrec_out_class_
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_sslrec_out_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

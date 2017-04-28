// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_sslio_context
{
	pub engine: *mut br_ssl_engine_context,
	pub low_read: Option<unsafe extern "C" fn(read_context: *mut c_void, data: *mut c_uchar, len: usize) -> c_int>,
	pub read_context: *mut c_void,
	pub low_write: Option<unsafe extern "C" fn(write_context: *mut c_void, data: *const c_uchar, len: usize) -> c_int>,
	pub write_context: *mut c_void,
}

impl Clone for br_sslio_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_sslio_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

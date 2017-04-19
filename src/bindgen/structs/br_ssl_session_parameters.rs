// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_ssl_session_parameters
{
	pub session_id: [c_uchar; 32usize],
	pub session_id_len: c_uchar,
	pub version: uint16_t,
	pub cipher_suite: uint16_t,
	pub master_secret: [c_uchar; 48usize],
}

impl Clone for br_ssl_session_parameters
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_session_parameters
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_ssl_server_choices
{
	pub cipher_suite: u16,
	pub algo_id: c_uint,
	pub chain: *const br_x509_certificate,
	pub chain_len: usize,
}

impl Clone for br_ssl_server_choices
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_server_choices
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

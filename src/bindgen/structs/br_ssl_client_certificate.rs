// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_ssl_client_certificate
{
	pub auth_type: c_int,
	pub hash_id: c_int,
	pub chain: *const br_x509_certificate,
	pub chain_len: usize,
}

impl Clone for br_ssl_client_certificate
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_client_certificate
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

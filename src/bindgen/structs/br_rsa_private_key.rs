// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_rsa_private_key
{
	pub n_bitlen: u32,
	pub p: *mut c_uchar,
	pub plen: usize,
	pub q: *mut c_uchar,
	pub qlen: usize,
	pub dp: *mut c_uchar,
	pub dplen: usize,
	pub dq: *mut c_uchar,
	pub dqlen: usize,
	pub iq: *mut c_uchar,
	pub iqlen: usize,
}

impl Clone for br_rsa_private_key
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_rsa_private_key
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

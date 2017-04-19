// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_rsa_private_key
{
	pub n_bitlen: uint32_t,
	pub p: *mut c_uchar,
	pub plen: size_t,
	pub q: *mut c_uchar,
	pub qlen: size_t,
	pub dp: *mut c_uchar,
	pub dplen: size_t,
	pub dq: *mut c_uchar,
	pub dqlen: size_t,
	pub iq: *mut c_uchar,
	pub iqlen: size_t,
}

impl Default for br_rsa_private_key
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

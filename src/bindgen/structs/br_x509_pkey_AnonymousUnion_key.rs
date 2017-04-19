// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_x509_pkey_AnonymousUnion_key
{
	pub _bindgen_data_: [u64; 4usize],
}

impl br_x509_pkey_AnonymousUnion_key
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn rsa(&mut self) -> *mut br_rsa_public_key
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn ec(&mut self) -> *mut br_ec_public_key
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Default for br_x509_pkey_AnonymousUnion_key
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

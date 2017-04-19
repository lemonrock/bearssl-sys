// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_client_context__AnonymousUnion_client_auth
{
	pub _bindgen_data_: [u64; 8usize],
}

impl br_ssl_client_context__AnonymousUnion_client_auth
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn vtable(&mut self) -> *mut *const br_ssl_client_certificate_class
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn single_rsa(&mut self) -> *mut br_ssl_client_certificate_rsa_context
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn single_ec(&mut self) -> *mut br_ssl_client_certificate_ec_context
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Default for br_ssl_client_context__AnonymousUnion_client_auth
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

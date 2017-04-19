// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_sslrec_gcm_context_AnonymousUnion_vtable
{
	pub _bindgen_data_: [u64; 1usize],
}

impl br_sslrec_gcm_context_AnonymousUnion_vtable
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn gen(&mut self) -> *mut *const c_void
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn in_(&mut self) -> *mut *const br_sslrec_in_gcm_class
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn out(&mut self) -> *mut *const br_sslrec_out_gcm_class
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Default for br_sslrec_gcm_context_AnonymousUnion_vtable
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

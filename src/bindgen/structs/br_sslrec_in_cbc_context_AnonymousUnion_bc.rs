// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[allow(missing_debug_implementations)]
#[allow(missing_copy_implementations)]
pub struct br_sslrec_in_cbc_context_AnonymousUnion_bc
{
	pub _bindgen_data_: [u64; 50usize],
}

impl br_sslrec_in_cbc_context_AnonymousUnion_bc
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn vtable(&mut self) -> *mut *const br_block_cbcdec_class
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn aes(&mut self) -> *mut br_aes_gen_cbcdec_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn des(&mut self) -> *mut br_des_gen_cbcdec_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Default for br_sslrec_in_cbc_context_AnonymousUnion_bc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

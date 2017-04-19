// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_aes_gen_cbcenc_keys
{
	pub _bindgen_data_: [u64; 32usize],
}

impl br_aes_gen_cbcenc_keys
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn vtable(&mut self) -> *mut *const br_block_cbcenc_class
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn c_big(&mut self) -> *mut br_aes_big_cbcenc_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn c_small(&mut self) -> *mut br_aes_small_cbcenc_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn c_ct(&mut self) -> *mut br_aes_ct_cbcenc_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn c_ct64(&mut self) -> *mut br_aes_ct64_cbcenc_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn c_x86ni(&mut self) -> *mut br_aes_x86ni_cbcenc_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn c_pwr8(&mut self) -> *mut br_aes_pwr8_cbcenc_keys
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Default for br_aes_gen_cbcenc_keys
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

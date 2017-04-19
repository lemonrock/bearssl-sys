// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_aes_pwr8_ctr_keys_AnonymousUnion_skey
{
	pub _bindgen_data_: [u8; 240usize],
}

impl br_aes_pwr8_ctr_keys_AnonymousUnion_skey
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn skni(&mut self) -> *mut [c_uchar; 240usize]
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Clone for br_aes_pwr8_ctr_keys_AnonymousUnion_skey
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_aes_pwr8_ctr_keys_AnonymousUnion_skey
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

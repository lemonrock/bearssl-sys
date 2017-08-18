// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_aead_class_
{
	pub tag_size: usize,
	pub reset: Option<unsafe extern "C" fn(cc: *mut *const br_aead_class, iv: *const c_void, len: usize)>,
	pub aad_inject: Option<unsafe extern "C" fn(cc: *mut *const br_aead_class, data: *const c_void, len: usize)>,
	pub flip: Option<unsafe extern "C" fn(cc: *mut *const br_aead_class)>,
	pub run: Option<unsafe extern "C" fn(cc: *mut *const br_aead_class, encrypt: c_int, data: *mut c_void, len: usize)>,
	pub get_tag: Option<unsafe extern "C" fn(cc: *mut *const br_aead_class, tag: *mut c_void)>,
	pub check_tag: Option<unsafe extern "C" fn(cc: *mut *const br_aead_class, tag: *const c_void) -> u32>,
}

impl Clone for br_aead_class_
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_aead_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

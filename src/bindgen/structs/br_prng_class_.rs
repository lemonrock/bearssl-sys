// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_prng_class_
{
	pub context_size: usize,
	pub init: Option<unsafe extern "C" fn(ctx: *mut *const br_prng_class, params: *const c_void, seed: *const c_void, seed_len: usize)>,
	pub generate: Option<unsafe extern "C" fn(ctx: *mut *const br_prng_class, out: *mut c_void, len: usize)>,
	pub update: Option<unsafe extern "C" fn(ctx: *mut *const br_prng_class, seed: *const c_void, seed_len: usize)>,
}

impl Clone for br_prng_class_
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_prng_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

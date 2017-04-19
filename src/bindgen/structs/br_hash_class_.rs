// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_hash_class_
{
	pub context_size: size_t,
	pub desc: uint32_t,
	pub init: Option<unsafe extern "C" fn(ctx: *mut *const br_hash_class)>,
	pub update: Option<unsafe extern "C" fn(ctx: *mut *const br_hash_class, data: *const c_void, len: size_t)>,
	pub out: Option<unsafe extern "C" fn(ctx: *const *const br_hash_class, dst: *mut c_void)>,
	pub state: Option<unsafe extern "C" fn(ctx: *const *const br_hash_class, dst: *mut c_void) -> uint64_t>,
	pub set_state: Option<unsafe extern "C" fn(ctx: *mut *const br_hash_class, stb: *const c_void, count: uint64_t)>,
}

impl Default for br_hash_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

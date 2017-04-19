// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ec_impl
{
	pub supported_curves: uint32_t,
	pub generator: Option<unsafe extern "C" fn(curve: c_int, len: *mut size_t) -> *const c_uchar>,
	pub order: Option<unsafe extern "C" fn(curve: c_int, len: *mut size_t) -> *const c_uchar>,
	pub xoff: Option<unsafe extern "C" fn(curve: c_int, len: *mut size_t) -> size_t>,
	pub mul: Option<unsafe extern "C" fn(G: *mut c_uchar, Glen: size_t, x: *const c_uchar, xlen: size_t, curve: c_int) -> uint32_t>,
	pub mulgen: Option<unsafe extern "C" fn(R: *mut c_uchar, x: *const c_uchar, xlen: size_t, curve: c_int) -> size_t>,
	pub muladd: Option<unsafe extern "C" fn(A: *mut c_uchar, B: *const c_uchar, len: size_t, x: *const c_uchar, xlen: size_t, y: *const c_uchar, ylen: size_t, curve: c_int) -> uint32_t>,
}

impl Default for br_ec_impl
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_ec_impl
{
	pub supported_curves: u32,
	pub generator: Option<unsafe extern "C" fn(curve: c_int, len: *mut usize) -> *const c_uchar>,
	pub order: Option<unsafe extern "C" fn(curve: c_int, len: *mut usize) -> *const c_uchar>,
	pub xoff: Option<unsafe extern "C" fn(curve: c_int, len: *mut usize) -> usize>,
	pub mul: Option<unsafe extern "C" fn(G: *mut c_uchar, Glen: usize, x: *const c_uchar, xlen: usize, curve: c_int) -> u32>,
	pub mulgen: Option<unsafe extern "C" fn(R: *mut c_uchar, x: *const c_uchar, xlen: usize, curve: c_int) -> usize>,
	pub muladd: Option<unsafe extern "C" fn(A: *mut c_uchar, B: *const c_uchar, len: usize, x: *const c_uchar, xlen: usize, y: *const c_uchar, ylen: usize, curve: c_int) -> u32>,
}

impl Clone for br_ec_impl
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ec_impl
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

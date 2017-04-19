// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_x509_class_
{
	pub context_size: size_t,
	pub start_chain: Option<unsafe extern "C" fn(ctx: *mut *const br_x509_class, server_name: *const c_char)>,
	pub start_cert: Option<unsafe extern "C" fn(ctx: *mut *const br_x509_class, length: uint32_t)>,
	pub append: Option<unsafe extern "C" fn(ctx: *mut *const br_x509_class, buf: *const c_uchar, len: size_t)>,
	pub end_cert: Option<unsafe extern "C" fn(ctx: *mut *const br_x509_class)>,
	pub end_chain: Option<unsafe extern "C" fn(ctx: *mut *const br_x509_class) -> c_uint>,
	pub get_pkey: Option<unsafe extern "C" fn(ctx: *const *const br_x509_class, usages: *mut c_uint) -> *const br_x509_pkey>,
}

impl Default for br_x509_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

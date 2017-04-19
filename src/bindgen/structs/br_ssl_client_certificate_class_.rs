// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_client_certificate_class_
{
	pub context_size: size_t,
	pub start_name_list: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class)>,
	pub start_name: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class, len: size_t)>,
	pub append_name: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class, data: *const c_uchar, len: size_t)>,
	pub end_name: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class)>,
	pub end_name_list: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class)>,
	pub choose: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class, cc: *const br_ssl_client_context, auth_types: uint32_t, choices: *mut br_ssl_client_certificate)>,
	pub do_keyx: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class, data: *mut c_uchar, len: *mut size_t) -> uint32_t>,
	pub do_sign: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_client_certificate_class, hash_id: c_int, hv_len: size_t, data: *mut c_uchar, len: size_t) -> size_t>,
}

impl Default for br_ssl_client_certificate_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

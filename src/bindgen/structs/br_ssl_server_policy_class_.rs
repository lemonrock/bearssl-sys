// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_server_policy_class_
{
	pub context_size: size_t,
	pub choose: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_server_policy_class, cc: *const br_ssl_server_context, choices: *mut br_ssl_server_choices) -> c_int>,
	pub do_keyx: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_server_policy_class, data: *mut c_uchar, len: *mut size_t) -> uint32_t>,
	pub do_sign: Option<unsafe extern "C" fn(pctx: *mut *const br_ssl_server_policy_class, algo_id: c_uint, data: *mut c_uchar, hv_len: size_t, len: size_t) -> size_t>,
}

impl Default for br_ssl_server_policy_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

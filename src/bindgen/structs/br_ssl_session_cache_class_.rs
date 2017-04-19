// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_session_cache_class_
{
	pub context_size: size_t,
	pub save: Option<unsafe extern "C" fn(ctx: *mut *const br_ssl_session_cache_class, server_ctx: *mut br_ssl_server_context, params: *const br_ssl_session_parameters)>,
	pub load: Option<unsafe extern "C" fn(ctx: *mut *const br_ssl_session_cache_class, server_ctx: *mut br_ssl_server_context, params: *mut br_ssl_session_parameters) -> c_int>,
}

impl Default for br_ssl_session_cache_class_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

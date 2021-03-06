// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_ssl_session_cache_lru_forget(cc: *mut br_ssl_session_cache_lru, id: *const c_uchar);
	pub fn br_ssl_session_cache_lru_init(cc: *mut br_ssl_session_cache_lru, store: *mut c_uchar, store_len: usize);
}

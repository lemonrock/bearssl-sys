// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_session_cache_lru
{
	pub vtable: *const br_ssl_session_cache_class,
	pub store: *mut c_uchar,
	pub store_len: size_t,
	pub store_ptr: size_t,
	pub index_key: [c_uchar; 32usize],
	pub hash: *const br_hash_class,
	pub init_done: c_int,
	pub head: uint32_t,
	pub tail: uint32_t,
	pub root: uint32_t,
}

impl Default for br_ssl_session_cache_lru
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

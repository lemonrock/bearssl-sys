// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub union br_hash_compat_context
{
    pub vtable: *const br_hash_class,
    pub md5: br_md5_context,
    pub sha1: br_sha1_context,
    pub sha224: br_sha224_context,
    pub sha256: br_sha256_context,
    pub sha384: br_sha384_context,
    pub sha512: br_sha512_context,
    pub md5sha1: br_md5sha1_context,
}

impl Default for br_hash_compat_context
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

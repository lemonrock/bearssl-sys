// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct br_hash_compat_context
{
	pub vtable: __BindgenUnionField<*const br_hash_class>,
	pub md5: __BindgenUnionField<br_md5_context>,
	pub sha1: __BindgenUnionField<br_sha1_context>,
	pub sha224: __BindgenUnionField<br_sha224_context>,
	pub sha256: __BindgenUnionField<br_sha256_context>,
	pub sha384: __BindgenUnionField<br_sha384_context>,
	pub sha512: __BindgenUnionField<br_sha512_context>,
	pub md5sha1: __BindgenUnionField<br_md5sha1_context>,
	pub bindgen_union_field: [u64; 26usize],
}

impl Clone for br_hash_compat_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

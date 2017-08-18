// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_des_gen_cbcenc_keys
{
	pub vtable: __BindgenUnionField<*const br_block_cbcenc_class>,
	pub tab: __BindgenUnionField<br_des_tab_cbcenc_keys>,
	pub ct: __BindgenUnionField<br_des_ct_cbcenc_keys>,
	pub bindgen_union_field: [u64; 50usize],
}

impl Clone for br_des_gen_cbcenc_keys
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_des_gen_cbcenc_keys
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

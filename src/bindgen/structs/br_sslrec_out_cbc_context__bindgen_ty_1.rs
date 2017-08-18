// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_sslrec_out_cbc_context__bindgen_ty_1
{
	pub vtable: __BindgenUnionField<*const br_block_cbcenc_class>,
	pub aes: __BindgenUnionField<br_aes_gen_cbcenc_keys>,
	pub des: __BindgenUnionField<br_des_gen_cbcenc_keys>,
	pub bindgen_union_field: [u64; 50usize],
}

impl Clone for br_sslrec_out_cbc_context__bindgen_ty_1
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_sslrec_out_cbc_context__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Default for br_sslrec_out_cbc_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct br_aes_gen_cbcdec_keys
{
	pub vtable: __BindgenUnionField<*const br_block_cbcdec_class>,
	pub c_big: __BindgenUnionField<br_aes_big_cbcdec_keys>,
	pub c_small: __BindgenUnionField<br_aes_small_cbcdec_keys>,
	pub c_ct: __BindgenUnionField<br_aes_ct_cbcdec_keys>,
	pub c_ct64: __BindgenUnionField<br_aes_ct64_cbcdec_keys>,
	pub c_x86ni: __BindgenUnionField<br_aes_x86ni_cbcdec_keys>,
	pub c_pwr8: __BindgenUnionField<br_aes_pwr8_cbcdec_keys>,
	pub bindgen_union_field: [u64; 32usize],
}

impl Clone for br_aes_gen_cbcdec_keys
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

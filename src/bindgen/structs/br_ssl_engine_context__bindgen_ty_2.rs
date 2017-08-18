// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_ssl_engine_context__bindgen_ty_2
{
	pub vtable: __BindgenUnionField<*const br_sslrec_out_class>,
	pub clear: __BindgenUnionField<br_sslrec_out_clear_context>,
	pub cbc: __BindgenUnionField<br_sslrec_out_cbc_context>,
	pub gcm: __BindgenUnionField<br_sslrec_gcm_context>,
	pub chapol: __BindgenUnionField<br_sslrec_chapol_context>,
	pub bindgen_union_field: [u64; 73usize],
}

impl Clone for br_ssl_engine_context__bindgen_ty_2
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_engine_context__bindgen_ty_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

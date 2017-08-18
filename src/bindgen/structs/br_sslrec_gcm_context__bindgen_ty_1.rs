// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct br_sslrec_gcm_context__bindgen_ty_1
{
	pub gen: __BindgenUnionField<*const c_void>,
	pub in_: __BindgenUnionField<*const br_sslrec_in_gcm_class>,
	pub out: __BindgenUnionField<*const br_sslrec_out_gcm_class>,
	pub bindgen_union_field: u64,
}

impl Clone for br_sslrec_gcm_context__bindgen_ty_1
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

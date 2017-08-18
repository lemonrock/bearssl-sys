// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct br_ssl_server_context___bindgen_ty_1
{
	pub vtable: __BindgenUnionField<*const br_ssl_server_policy_class>,
	pub single_rsa: __BindgenUnionField<br_ssl_server_policy_rsa_context>,
	pub single_ec: __BindgenUnionField<br_ssl_server_policy_ec_context>,
	pub bindgen_union_field: [u64; 8usize],
}

impl Clone for br_ssl_server_context___bindgen_ty_1
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_server_context_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

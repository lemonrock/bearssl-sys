// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union br_ssl_client_context___bindgen_ty_1
{
    pub vtable: *const br_ssl_client_certificate_class,
    pub single_rsa: br_ssl_client_certificate_rsa_context,
    pub single_ec: br_ssl_client_certificate_ec_context,
}

impl Clone for br_ssl_client_context___bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_client_context___bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Default for br_ssl_client_context_
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

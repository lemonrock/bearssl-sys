// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union br_x509_pkey__bindgen_ty_1
{
    pub rsa: br_rsa_public_key,
    pub ec: br_ec_public_key,
}

impl Clone for br_x509_pkey__bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_x509_pkey__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for br_x509_pkey
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_x509_pkey
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

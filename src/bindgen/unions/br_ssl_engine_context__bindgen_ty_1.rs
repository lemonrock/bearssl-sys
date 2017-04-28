// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub union br_ssl_engine_context__bindgen_ty_1
{
    pub vtable: *const br_sslrec_in_class,
    pub cbc: br_sslrec_in_cbc_context,
    pub gcm: br_sslrec_gcm_context,
    pub chapol: br_sslrec_chapol_context,
}

impl Default for br_ssl_engine_context__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

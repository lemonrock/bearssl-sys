// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub union br_des_gen_cbcdec_keys
{
    pub vtable: *const br_block_cbcdec_class,
    pub c_tab: br_des_tab_cbcdec_keys,
    pub c_ct: br_des_ct_cbcdec_keys,
}

impl Default for br_des_gen_cbcdec_keys
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

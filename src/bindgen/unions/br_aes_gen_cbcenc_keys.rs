// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub union br_aes_gen_cbcenc_keys
{
    pub vtable: *const br_block_cbcenc_class,
    pub c_big: br_aes_big_cbcenc_keys,
    pub c_small: br_aes_small_cbcenc_keys,
    pub c_ct: br_aes_ct_cbcenc_keys,
    pub c_ct64: br_aes_ct64_cbcenc_keys,
    pub c_x86ni: br_aes_x86ni_cbcenc_keys,
    pub c_pwr8: br_aes_pwr8_cbcenc_keys,
}

impl Default for br_aes_gen_cbcenc_keys
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

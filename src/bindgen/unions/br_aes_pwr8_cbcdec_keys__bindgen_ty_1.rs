// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub union br_aes_pwr8_cbcdec_keys__bindgen_ty_1
{
    pub skni: [c_uchar; 240usize],
}

impl Default for br_aes_pwr8_cbcdec_keys__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Default for br_aes_pwr8_cbcdec_keys
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

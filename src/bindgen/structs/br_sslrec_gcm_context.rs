// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct br_sslrec_gcm_context
{
	pub vtable: br_sslrec_gcm_context__bindgen_ty_1,
	pub seq: u64,
	pub bc: br_sslrec_gcm_context__bindgen_ty_2,
	pub gh: br_ghash,
	pub iv: [c_uchar; 4usize],
	pub h: [c_uchar; 16usize],
}

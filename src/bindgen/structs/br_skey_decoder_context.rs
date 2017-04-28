// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub struct br_skey_decoder_context
{
	pub key: br_skey_decoder_context__bindgen_ty_1,
	pub cpu: br_skey_decoder_context__bindgen_ty_2,
	pub dp_stack: [u32; 32usize],
	pub rp_stack: [u32; 32usize],
	pub err: c_int,
	pub hbuf: *const c_uchar,
	pub hlen: usize,
	pub pad: [c_uchar; 256usize],
	pub key_type: c_uchar,
	pub key_data: [c_uchar; 1536usize],
}

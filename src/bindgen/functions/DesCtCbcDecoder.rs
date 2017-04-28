// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_des_ct_cbcdec_init(ctx: *mut br_des_ct_cbcdec_keys, key: *const c_void, len: usize);
	pub fn br_des_ct_cbcdec_run(ctx: *const br_des_ct_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: usize);
	pub fn br_des_ct_cbcenc_init(ctx: *mut br_des_ct_cbcenc_keys, key: *const c_void, len: usize);
	pub fn br_des_ct_cbcenc_run(ctx: *const br_des_ct_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: usize);
}

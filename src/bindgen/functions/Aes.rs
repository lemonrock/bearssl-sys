// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_aes_big_cbcdec_init(ctx: *mut br_aes_big_cbcdec_keys, key: *const c_void, len: size_t);
	pub fn br_aes_big_cbcdec_run(ctx: *const br_aes_big_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_big_cbcenc_init(ctx: *mut br_aes_big_cbcenc_keys, key: *const c_void, len: size_t);
	pub fn br_aes_big_cbcenc_run(ctx: *const br_aes_big_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_big_ctr_init(ctx: *mut br_aes_big_ctr_keys, key: *const c_void, len: size_t);
	pub fn br_aes_big_ctr_run(ctx: *const br_aes_big_ctr_keys, iv: *const c_void, cc: uint32_t, data: *mut c_void, len: size_t) -> uint32_t;
	pub fn br_aes_ct64_cbcdec_init(ctx: *mut br_aes_ct64_cbcdec_keys, key: *const c_void, len: size_t);
	pub fn br_aes_ct64_cbcdec_run(ctx: *const br_aes_ct64_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_ct64_cbcenc_init(ctx: *mut br_aes_ct64_cbcenc_keys, key: *const c_void, len: size_t);
	pub fn br_aes_ct64_cbcenc_run(ctx: *const br_aes_ct64_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_ct64_ctr_init(ctx: *mut br_aes_ct64_ctr_keys, key: *const c_void, len: size_t);
	pub fn br_aes_ct64_ctr_run(ctx: *const br_aes_ct64_ctr_keys, iv: *const c_void, cc: uint32_t, data: *mut c_void, len: size_t) -> uint32_t;
	pub fn br_aes_ct_cbcdec_init(ctx: *mut br_aes_ct_cbcdec_keys, key: *const c_void, len: size_t);
	pub fn br_aes_ct_cbcdec_run(ctx: *const br_aes_ct_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_ct_cbcenc_init(ctx: *mut br_aes_ct_cbcenc_keys, key: *const c_void, len: size_t);
	pub fn br_aes_ct_cbcenc_run(ctx: *const br_aes_ct_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_ct_ctr_init(ctx: *mut br_aes_ct_ctr_keys, key: *const c_void, len: size_t);
	pub fn br_aes_ct_ctr_run(ctx: *const br_aes_ct_ctr_keys, iv: *const c_void, cc: uint32_t, data: *mut c_void, len: size_t) -> uint32_t;
	pub fn br_aes_pwr8_cbcdec_get_vtable() -> *const br_block_cbcdec_class;
	pub fn br_aes_pwr8_cbcdec_init(ctx: *mut br_aes_pwr8_cbcdec_keys, key: *const c_void, len: size_t);
	pub fn br_aes_pwr8_cbcdec_run(ctx: *const br_aes_pwr8_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_pwr8_cbcenc_get_vtable() -> *const br_block_cbcenc_class;
	pub fn br_aes_pwr8_cbcenc_init(ctx: *mut br_aes_pwr8_cbcenc_keys, key: *const c_void, len: size_t);
	pub fn br_aes_pwr8_cbcenc_run(ctx: *const br_aes_pwr8_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_pwr8_ctr_get_vtable() -> *const br_block_ctr_class;
	pub fn br_aes_pwr8_ctr_init(ctx: *mut br_aes_pwr8_ctr_keys, key: *const c_void, len: size_t);
	pub fn br_aes_pwr8_ctr_run(ctx: *const br_aes_pwr8_ctr_keys, iv: *const c_void, cc: uint32_t, data: *mut c_void, len: size_t) -> uint32_t;
	pub fn br_aes_small_cbcdec_init(ctx: *mut br_aes_small_cbcdec_keys, key: *const c_void, len: size_t);
	pub fn br_aes_small_cbcdec_run(ctx: *const br_aes_small_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_small_cbcenc_init(ctx: *mut br_aes_small_cbcenc_keys, key: *const c_void, len: size_t);
	pub fn br_aes_small_cbcenc_run(ctx: *const br_aes_small_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_small_ctr_init(ctx: *mut br_aes_small_ctr_keys, key: *const c_void, len: size_t);
	pub fn br_aes_small_ctr_run(ctx: *const br_aes_small_ctr_keys, iv: *const c_void, cc: uint32_t, data: *mut c_void, len: size_t) -> uint32_t;
	pub fn br_aes_x86ni_cbcdec_get_vtable() -> *const br_block_cbcdec_class;
	pub fn br_aes_x86ni_cbcdec_init(ctx: *mut br_aes_x86ni_cbcdec_keys, key: *const c_void, len: size_t);
	pub fn br_aes_x86ni_cbcdec_run(ctx: *const br_aes_x86ni_cbcdec_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_x86ni_cbcenc_get_vtable() -> *const br_block_cbcenc_class;
	pub fn br_aes_x86ni_cbcenc_init(ctx: *mut br_aes_x86ni_cbcenc_keys, key: *const c_void, len: size_t);
	pub fn br_aes_x86ni_cbcenc_run(ctx: *const br_aes_x86ni_cbcenc_keys, iv: *mut c_void, data: *mut c_void, len: size_t);
	pub fn br_aes_x86ni_ctr_get_vtable() -> *const br_block_ctr_class;
	pub fn br_aes_x86ni_ctr_init(ctx: *mut br_aes_x86ni_ctr_keys, key: *const c_void, len: size_t);
	pub fn br_aes_x86ni_ctr_run(ctx: *const br_aes_x86ni_ctr_keys, iv: *const c_void, cc: uint32_t, data: *mut c_void, len: size_t) -> uint32_t;
}

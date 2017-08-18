// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_gcm_aad_inject(ctx: *mut br_gcm_context, data: *const c_void, len: usize);
	pub fn br_gcm_check_tag(ctx: *mut br_gcm_context, tag: *const c_void) -> u32;
	pub fn br_gcm_flip(ctx: *mut br_gcm_context);
	pub fn br_gcm_get_tag(ctx: *mut br_gcm_context, tag: *mut c_void);
	pub fn br_gcm_init(ctx: *mut br_gcm_context, bctx: *mut *const br_block_ctr_class, gh: br_ghash);
	pub fn br_gcm_reset(ctx: *mut br_gcm_context, iv: *const c_void, len: usize);
	pub fn br_gcm_run(ctx: *mut br_gcm_context, encrypt: c_int, data: *mut c_void, len: usize);
	pub fn br_ssl_key_export(cc: *mut br_ssl_engine_context, dst: *mut c_void, len: usize, label: *const c_char, context: *const c_void, context_len: usize) -> c_int;
}

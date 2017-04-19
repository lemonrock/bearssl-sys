// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_hmac_init(ctx: *mut br_hmac_context, kc: *const br_hmac_key_context, out_len: size_t);
	pub fn br_hmac_key_init(kc: *mut br_hmac_key_context, digest_vtable: *const br_hash_class, key: *const c_void, key_len: size_t);
	pub fn br_hmac_out(ctx: *const br_hmac_context, out: *mut c_void) -> size_t;
	pub fn br_hmac_outCT(ctx: *const br_hmac_context, data: *const c_void, len: size_t, min_len: size_t, max_len: size_t, out: *mut c_void) -> size_t;
	pub fn br_hmac_update(ctx: *mut br_hmac_context, data: *const c_void, len: size_t);
}

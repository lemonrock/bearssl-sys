// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_hmac_drbg_generate(ctx: *mut br_hmac_drbg_context, out: *mut c_void, len: usize);
	pub fn br_hmac_drbg_init(ctx: *mut br_hmac_drbg_context, digest_class: *const br_hash_class, seed: *const c_void, seed_len: usize);
	pub fn br_hmac_drbg_update(ctx: *mut br_hmac_drbg_context, seed: *const c_void, seed_len: usize);
}

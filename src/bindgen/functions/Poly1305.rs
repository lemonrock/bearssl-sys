// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_poly1305_ctmul32_run(key: *const c_void, iv: *const c_void, data: *mut c_void, len: size_t, aad: *const c_void, aad_len: size_t, tag: *mut c_void, ichacha: br_chacha20_run, encrypt: c_int);
	pub fn br_poly1305_ctmul_run(key: *const c_void, iv: *const c_void, data: *mut c_void, len: size_t, aad: *const c_void, aad_len: size_t, tag: *mut c_void, ichacha: br_chacha20_run, encrypt: c_int);
	pub fn br_poly1305_ctmulq_get() -> br_poly1305_run;
	pub fn br_poly1305_ctmulq_run(key: *const c_void, iv: *const c_void, data: *mut c_void, len: size_t, aad: *const c_void, aad_len: size_t, tag: *mut c_void, ichacha: br_chacha20_run, encrypt: c_int);
	pub fn br_poly1305_i15_run(key: *const c_void, iv: *const c_void, data: *mut c_void, len: size_t, aad: *const c_void, aad_len: size_t, tag: *mut c_void, ichacha: br_chacha20_run, encrypt: c_int);
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_tls12_sha256_prf(dst: *mut c_void, len: size_t, secret: *const c_void, secret_len: size_t, label: *const c_char, seed: *const c_void, seed_len: size_t);
	pub fn br_tls12_sha384_prf(dst: *mut c_void, len: size_t, secret: *const c_void, secret_len: size_t, label: *const c_char, seed: *const c_void, seed_len: size_t);
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_ghash_ctmul(y: *mut c_void, h: *const c_void, data: *const c_void, len: size_t);
	pub fn br_ghash_ctmul32(y: *mut c_void, h: *const c_void, data: *const c_void, len: size_t);
	pub fn br_ghash_ctmul64(y: *mut c_void, h: *const c_void, data: *const c_void, len: size_t);
	pub fn br_ghash_pclmul(y: *mut c_void, h: *const c_void, data: *const c_void, len: size_t);
	pub fn br_ghash_pclmul_get() -> br_ghash;
	pub fn br_ghash_pwr8(y: *mut c_void, h: *const c_void, data: *const c_void, len: size_t);
	pub fn br_ghash_pwr8_get() -> br_ghash;
}

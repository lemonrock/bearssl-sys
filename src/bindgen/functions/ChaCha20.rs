// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_chacha20_ct_run(key: *const c_void, iv: *const c_void, cc: u32, data: *mut c_void, len: usize) -> u32;
	pub fn br_chacha20_sse2_get() -> br_chacha20_run;
	pub fn br_chacha20_sse2_run(key: *const c_void, iv: *const c_void, cc: u32, data: *mut c_void, len: usize) -> u32;
}

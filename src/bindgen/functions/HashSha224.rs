// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_sha224_init(ctx: *mut br_sha224_context);
	pub fn br_sha224_out(ctx: *const br_sha224_context, out: *mut c_void);
	pub fn br_sha224_set_state(ctx: *mut br_sha224_context, stb: *const c_void, count: u64);
	pub fn br_sha224_state(ctx: *const br_sha224_context, out: *mut c_void) -> u64;
	pub fn br_sha224_update(ctx: *mut br_sha224_context, data: *const c_void, len: usize);
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_multihash_init(ctx: *mut br_multihash_context);
	pub fn br_multihash_out(ctx: *const br_multihash_context, id: c_int, dst: *mut c_void) -> size_t;
	pub fn br_multihash_update(ctx: *mut br_multihash_context, data: *const c_void, len: size_t);
	pub fn br_multihash_zero(ctx: *mut br_multihash_context);
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_x509_decoder_init(ctx: *mut br_x509_decoder_context, append_dn: Option<unsafe extern "C" fn(ctx: *mut c_void, buf: *const c_void, len: usize)>, append_dn_ctx: *mut c_void);
	pub fn br_x509_decoder_push(ctx: *mut br_x509_decoder_context, data: *const c_void, len: usize);
}

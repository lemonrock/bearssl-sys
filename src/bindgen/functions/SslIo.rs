// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_sslio_close(cc: *mut br_sslio_context) -> c_int;
	pub fn br_sslio_flush(cc: *mut br_sslio_context) -> c_int;
	pub fn br_sslio_init(ctx: *mut br_sslio_context, engine: *mut br_ssl_engine_context, low_read: Option<unsafe extern "C" fn(read_context: *mut c_void, data: *mut c_uchar, len: size_t) -> c_int>, read_context: *mut c_void, low_write: Option<unsafe extern "C" fn(write_context: *mut c_void, data: *const c_uchar, len: size_t) -> c_int>, write_context: *mut c_void);
	pub fn br_sslio_read(cc: *mut br_sslio_context, dst: *mut c_void, len: size_t) -> c_int;
	pub fn br_sslio_read_all(cc: *mut br_sslio_context, dst: *mut c_void, len: size_t) -> c_int;
	pub fn br_sslio_write(cc: *mut br_sslio_context, src: *const c_void, len: size_t) -> c_int;
	pub fn br_sslio_write_all(cc: *mut br_sslio_context, src: *const c_void, len: size_t) -> c_int;
}

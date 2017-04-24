// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct br_pem_decoder_context
{
	pub cpu: br_pem_decoder_context_AnonymousStruct_cpu,
	pub dp_stack: [uint32_t; 32usize],
	pub rp_stack: [uint32_t; 32usize],
	pub err: c_int,
	pub hbuf: *const c_uchar,
	pub hlen: size_t,
	pub dest: Option<unsafe extern "C" fn(dest_ctx: *mut c_void, src: *const c_void, len: size_t)>,
	pub dest_ctx: *mut c_void,
	pub event: c_uchar,
	pub name: [c_char; 128usize],
	pub buf: [c_uchar; 255usize],
	pub ptr: size_t,
}

impl Clone for br_pem_decoder_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_pem_decoder_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

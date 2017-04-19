// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_x509_decoder_context
{
	pub pkey: br_x509_pkey,
	pub cpu: br_x509_decoder_context_AnonymousStruct_cpu,
	pub dp_stack: [uint32_t; 32usize],
	pub rp_stack: [uint32_t; 32usize],
	pub err: c_int,
	pub pad: [c_uchar; 256usize],
	pub decoded: c_uchar,
	pub notbefore_days: uint32_t,
	pub notbefore_seconds: uint32_t,
	pub notafter_days: uint32_t,
	pub notafter_seconds: uint32_t,
	pub isCA: c_uchar,
	pub copy_dn: c_uchar,
	pub append_dn_ctx: *mut c_void,
	pub append_dn: Option<unsafe extern "C" fn(ctx: *mut c_void, buf: *const c_void, len: size_t)>,
	pub hbuf: *const c_uchar,
	pub hlen: size_t,
	pub pkey_data: [c_uchar; 520usize],
	pub signer_key_type: c_uchar,
	pub signer_hash_id: c_uchar,
}

impl Clone for br_x509_decoder_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_x509_decoder_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

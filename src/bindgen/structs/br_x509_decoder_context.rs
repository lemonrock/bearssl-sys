// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub struct br_x509_decoder_context
{
	pub pkey: br_x509_pkey,
	pub cpu: br_x509_decoder_context__bindgen_ty_1,
	pub dp_stack: [u32; 32usize],
	pub rp_stack: [u32; 32usize],
	pub err: c_int,
	pub pad: [c_uchar; 256usize],
	pub decoded: c_uchar,
	pub notbefore_days: u32,
	pub notbefore_seconds: u32,
	pub notafter_days: u32,
	pub notafter_seconds: u32,
	pub isCA: c_uchar,
	pub copy_dn: c_uchar,
	pub append_dn_ctx: *mut c_void,
	pub append_dn: Option<unsafe extern "C" fn(ctx: *mut c_void, buf: *const c_void, len: usize)>,
	pub hbuf: *const c_uchar,
	pub hlen: usize,
	pub pkey_data: [c_uchar; 520usize],
	pub signer_key_type: c_uchar,
	pub signer_hash_id: c_uchar,
}

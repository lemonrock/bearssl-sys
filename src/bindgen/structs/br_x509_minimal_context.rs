// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_x509_minimal_context
{
	pub vtable: *const br_x509_class,
	pub pkey: br_x509_pkey,
	pub cpu: br_x509_minimal_context_AnonymousStruct_cpu,
	pub dp_stack: [uint32_t; 32usize],
	pub rp_stack: [uint32_t; 32usize],
	pub err: c_int,
	pub server_name: *const c_char,
	pub key_usages: c_uchar,
	pub days: uint32_t,
	pub seconds: uint32_t,
	pub cert_length: uint32_t,
	pub num_certs: uint32_t,
	pub hbuf: *const c_uchar,
	pub hlen: size_t,
	pub pad: [c_uchar; 256usize],
	pub ee_pkey_data: [c_uchar; 520usize],
	pub pkey_data: [c_uchar; 520usize],
	pub cert_signer_key_type: c_uchar,
	pub cert_sig_hash_oid: uint16_t,
	pub cert_sig_hash_len: c_uchar,
	pub cert_sig: [c_uchar; 512usize],
	pub cert_sig_len: uint16_t,
	pub min_rsa_size: int16_t,
	pub trust_anchors: *const br_x509_trust_anchor,
	pub trust_anchors_num: size_t,
	pub do_mhash: c_uchar,
	pub mhash: br_multihash_context,
	pub tbs_hash: [c_uchar; 64usize],
	pub do_dn_hash: c_uchar,
	pub dn_hash_impl: *const br_hash_class,
	pub dn_hash: br_hash_compat_context,
	pub current_dn_hash: [c_uchar; 64usize],
	pub next_dn_hash: [c_uchar; 64usize],
	pub saved_dn_hash: [c_uchar; 64usize],
	pub name_elts: *mut br_name_element,
	pub num_name_elts: size_t,
	pub irsa: br_rsa_pkcs1_vrfy,
	pub iecdsa: br_ecdsa_vrfy,
	pub iec: *const br_ec_impl,
}

impl Clone for br_x509_minimal_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_x509_minimal_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

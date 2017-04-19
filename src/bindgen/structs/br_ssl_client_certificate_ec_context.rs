// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_ssl_client_certificate_ec_context
{
	pub vtable: *const br_ssl_client_certificate_class,
	pub chain: *const br_x509_certificate,
	pub chain_len: size_t,
	pub sk: *const br_ec_private_key,
	pub allowed_usages: c_uint,
	pub issuer_key_type: c_uint,
	pub mhash: *const br_multihash_context,
	pub iec: *const br_ec_impl,
	pub iecdsa: br_ecdsa_sign,
}

impl Default for br_ssl_client_certificate_ec_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

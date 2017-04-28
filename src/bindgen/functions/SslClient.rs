// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_ssl_client_init_full(cc: *mut br_ssl_client_context, xc: *mut br_x509_minimal_context, trust_anchors: *const br_x509_trust_anchor, trust_anchors_num: usize);
	pub fn br_ssl_client_reset(cc: *mut br_ssl_client_context, server_name: *const c_char, resume_session: c_int) -> c_int;
	pub fn br_ssl_client_set_default_rsapub(cc: *mut br_ssl_client_context);
	pub fn br_ssl_client_set_single_ec(cc: *mut br_ssl_client_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_ec_private_key, allowed_usages: c_uint, cert_issuer_key_type: c_uint, iec: *const br_ec_impl, iecdsa: br_ecdsa_sign);
	pub fn br_ssl_client_set_single_rsa(cc: *mut br_ssl_client_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_rsa_private_key, irsasign: br_rsa_pkcs1_sign);
	pub fn br_ssl_client_zero(cc: *mut br_ssl_client_context);
}

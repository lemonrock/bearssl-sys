// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_ssl_server_init_full_ec(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, cert_issuer_key_type: c_uint, sk: *const br_ec_private_key);
	pub fn br_ssl_server_init_full_rsa(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_rsa_private_key);
	pub fn br_ssl_server_init_mine2c(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_rsa_private_key);
	pub fn br_ssl_server_init_mine2g(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_rsa_private_key);
	pub fn br_ssl_server_init_minf2c(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_ec_private_key);
	pub fn br_ssl_server_init_minf2g(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_ec_private_key);
	pub fn br_ssl_server_init_minr2g(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_rsa_private_key);
	pub fn br_ssl_server_init_minu2g(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_ec_private_key);
	pub fn br_ssl_server_init_minv2g(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_ec_private_key);
	pub fn br_ssl_server_reset(cc: *mut br_ssl_server_context) -> c_int;
	pub fn br_ssl_server_set_single_ec(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_ec_private_key, allowed_usages: c_uint, cert_issuer_key_type: c_uint, iec: *const br_ec_impl, iecdsa: br_ecdsa_sign);
	pub fn br_ssl_server_set_single_rsa(cc: *mut br_ssl_server_context, chain: *const br_x509_certificate, chain_len: usize, sk: *const br_rsa_private_key, allowed_usages: c_uint, irsacore: br_rsa_private, irsasign: br_rsa_pkcs1_sign);
	pub fn br_ssl_server_zero(cc: *mut br_ssl_server_context);
}

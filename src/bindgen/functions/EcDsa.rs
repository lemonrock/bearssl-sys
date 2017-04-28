// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_ecdsa_asn1_to_raw(sig: *mut c_void, sig_len: usize) -> usize;
	pub fn br_ecdsa_i15_sign_asn1(impl_: *const br_ec_impl, hf: *const br_hash_class, hash_value: *const c_void, sk: *const br_ec_private_key, sig: *mut c_void) -> usize;
	pub fn br_ecdsa_i15_sign_raw(impl_: *const br_ec_impl, hf: *const br_hash_class, hash_value: *const c_void, sk: *const br_ec_private_key, sig: *mut c_void) -> usize;
	pub fn br_ecdsa_i15_vrfy_asn1(impl_: *const br_ec_impl, hash: *const c_void, hash_len: usize, pk: *const br_ec_public_key, sig: *const c_void, sig_len: usize) -> u32;
	pub fn br_ecdsa_i15_vrfy_raw(impl_: *const br_ec_impl, hash: *const c_void, hash_len: usize, pk: *const br_ec_public_key, sig: *const c_void, sig_len: usize) -> u32;
	pub fn br_ecdsa_i31_sign_asn1(impl_: *const br_ec_impl, hf: *const br_hash_class, hash_value: *const c_void, sk: *const br_ec_private_key, sig: *mut c_void) -> usize;
	pub fn br_ecdsa_i31_sign_raw(impl_: *const br_ec_impl, hf: *const br_hash_class, hash_value: *const c_void, sk: *const br_ec_private_key, sig: *mut c_void) -> usize;
	pub fn br_ecdsa_i31_vrfy_asn1(impl_: *const br_ec_impl, hash: *const c_void, hash_len: usize, pk: *const br_ec_public_key, sig: *const c_void, sig_len: usize) -> u32;
	pub fn br_ecdsa_i31_vrfy_raw(impl_: *const br_ec_impl, hash: *const c_void, hash_len: usize, pk: *const br_ec_public_key, sig: *const c_void, sig_len: usize) -> u32;
	pub fn br_ecdsa_raw_to_asn1(sig: *mut c_void, sig_len: usize) -> usize;
	pub fn br_ecdsa_sign_asn1_get_default() -> br_ecdsa_sign;
	pub fn br_ecdsa_sign_raw_get_default() -> br_ecdsa_sign;
	pub fn br_ecdsa_vrfy_asn1_get_default() -> br_ecdsa_vrfy;
	pub fn br_ecdsa_vrfy_raw_get_default() -> br_ecdsa_vrfy;
}

// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_rsa_i15_pkcs1_sign(hash_oid: *const c_uchar, hash: *const c_uchar, hash_len: size_t, sk: *const br_rsa_private_key, x: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i15_pkcs1_vrfy(x: *const c_uchar, xlen: size_t, hash_oid: *const c_uchar, hash_len: size_t, pk: *const br_rsa_public_key, hash_out: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i15_private(x: *mut c_uchar, sk: *const br_rsa_private_key) -> uint32_t;
	pub fn br_rsa_i15_public(x: *mut c_uchar, xlen: size_t, pk: *const br_rsa_public_key) -> uint32_t;
	pub fn br_rsa_i31_pkcs1_sign(hash_oid: *const c_uchar, hash: *const c_uchar, hash_len: size_t, sk: *const br_rsa_private_key, x: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i31_pkcs1_vrfy(x: *const c_uchar, xlen: size_t, hash_oid: *const c_uchar, hash_len: size_t, pk: *const br_rsa_public_key, hash_out: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i31_private(x: *mut c_uchar, sk: *const br_rsa_private_key) -> uint32_t;
	pub fn br_rsa_i31_public(x: *mut c_uchar, xlen: size_t, pk: *const br_rsa_public_key) -> uint32_t;
	pub fn br_rsa_i32_pkcs1_sign(hash_oid: *const c_uchar, hash: *const c_uchar, hash_len: size_t, sk: *const br_rsa_private_key, x: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i32_pkcs1_vrfy(x: *const c_uchar, xlen: size_t, hash_oid: *const c_uchar, hash_len: size_t, pk: *const br_rsa_public_key, hash_out: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i32_private(x: *mut c_uchar, sk: *const br_rsa_private_key) -> uint32_t;
	pub fn br_rsa_i32_public(x: *mut c_uchar, xlen: size_t, pk: *const br_rsa_public_key) -> uint32_t;
	pub fn br_rsa_i62_pkcs1_sign(hash_oid: *const c_uchar, hash: *const c_uchar, hash_len: size_t, sk: *const br_rsa_private_key, x: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i62_pkcs1_sign_get() -> br_rsa_pkcs1_sign;
	pub fn br_rsa_i62_pkcs1_vrfy(x: *const c_uchar, xlen: size_t, hash_oid: *const c_uchar, hash_len: size_t, pk: *const br_rsa_public_key, hash_out: *mut c_uchar) -> uint32_t;
	pub fn br_rsa_i62_pkcs1_vrfy_get() -> br_rsa_pkcs1_vrfy;
	pub fn br_rsa_i62_private(x: *mut c_uchar, sk: *const br_rsa_private_key) -> uint32_t;
	pub fn br_rsa_i62_private_get() -> br_rsa_private;
	pub fn br_rsa_i62_public(x: *mut c_uchar, xlen: size_t, pk: *const br_rsa_public_key) -> uint32_t;
	pub fn br_rsa_i62_public_get() -> br_rsa_public;
	pub fn br_rsa_pkcs1_sign_get_default() -> br_rsa_pkcs1_sign;
	pub fn br_rsa_pkcs1_vrfy_get_default() -> br_rsa_pkcs1_vrfy;
	pub fn br_rsa_private_get_default() -> br_rsa_private;
	pub fn br_rsa_public_get_default() -> br_rsa_public;
	pub fn br_rsa_ssl_decrypt(core: br_rsa_private, sk: *const br_rsa_private_key, data: *mut c_uchar, len: size_t) -> uint32_t;
}

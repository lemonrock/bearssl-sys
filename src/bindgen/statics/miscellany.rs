// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub static br_aes_big_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_aes_big_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_aes_big_ctr_vtable: br_block_ctr_class;
	pub static br_aes_ct64_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_aes_ct64_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_aes_ct64_ctr_vtable: br_block_ctr_class;
	pub static br_aes_ct_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_aes_ct_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_aes_ct_ctr_vtable: br_block_ctr_class;
	pub static br_aes_pwr8_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_aes_pwr8_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_aes_pwr8_ctr_vtable: br_block_ctr_class;
	pub static br_aes_small_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_aes_small_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_aes_small_ctr_vtable: br_block_ctr_class;
	pub static br_aes_x86ni_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_aes_x86ni_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_aes_x86ni_ctr_vtable: br_block_ctr_class;
	pub static br_des_ct_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_des_ct_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_des_tab_cbcdec_vtable: br_block_cbcdec_class;
	pub static br_des_tab_cbcenc_vtable: br_block_cbcenc_class;
	pub static br_ec_all_m15: br_ec_impl;
	pub static br_ec_all_m31: br_ec_impl;
	pub static br_ec_c25519_i15: br_ec_impl;
	pub static br_ec_c25519_i31: br_ec_impl;
	pub static br_ec_c25519_m15: br_ec_impl;
	pub static br_ec_c25519_m31: br_ec_impl;
	pub static br_ec_p256_m15: br_ec_impl;
	pub static br_ec_p256_m31: br_ec_impl;
	pub static br_ec_prime_i15: br_ec_impl;
	pub static br_ec_prime_i31: br_ec_impl;
	pub static br_hmac_drbg_vtable: br_prng_class;
	pub static br_md5_vtable: br_hash_class;
	pub static br_md5sha1_vtable: br_hash_class;
	pub static br_sha1_vtable: br_hash_class;
	pub static br_sha224_vtable: br_hash_class;
	pub static br_sha256_vtable: br_hash_class;
	pub static br_sha384_vtable: br_hash_class;
	pub static br_sha512_vtable: br_hash_class;
	pub static br_sslrec_in_cbc_vtable: br_sslrec_in_cbc_class;
	pub static br_sslrec_in_chapol_vtable: br_sslrec_in_chapol_class;
	pub static br_sslrec_in_gcm_vtable: br_sslrec_in_gcm_class;
	pub static br_sslrec_out_cbc_vtable: br_sslrec_out_cbc_class;
	pub static br_sslrec_out_chapol_vtable: br_sslrec_out_chapol_class;
	pub static br_sslrec_out_clear_vtable: br_sslrec_out_class;
	pub static br_sslrec_out_gcm_vtable: br_sslrec_out_gcm_class;
	pub static br_x509_knownkey_vtable: br_x509_class;
	pub static br_x509_minimal_vtable: br_x509_class;
	pub static br_x509_minimal_vtable: br_x509_class;
}

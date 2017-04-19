// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct br_ssl_engine_context
{
	pub err: c_int,
	pub ibuf: *mut c_uchar,
	pub obuf: *mut c_uchar,
	pub ibuf_len: size_t,
	pub obuf_len: size_t,
	pub max_frag_len: uint16_t,
	pub log_max_frag_len: c_uchar,
	pub peer_log_max_frag_len: c_uchar,
	pub ixa: size_t,
	pub ixb: size_t,
	pub ixc: size_t,
	pub oxa: size_t,
	pub oxb: size_t,
	pub oxc: size_t,
	pub iomode: c_uchar,
	pub incrypt: c_uchar,
	pub shutdown_recv: c_uchar,
	pub record_type_in: c_uchar,
	pub record_type_out: c_uchar,
	pub version_in: uint16_t,
	pub version_out: uint16_t,
	pub in_: br_ssl_engine_context_AnonymousUnion_in_,
	pub out: br_ssl_engine_context_AnonymousUnion_out,
	pub application_data: c_uchar,
	pub rng: br_hmac_drbg_context,
	pub rng_init_done: c_int,
	pub rng_os_rand_done: c_int,
	pub version_min: uint16_t,
	pub version_max: uint16_t,
	pub suites_buf: [uint16_t; 40usize],
	pub suites_num: c_uchar,
	pub server_name: [c_char; 256usize],
	pub client_random: [c_uchar; 32usize],
	pub server_random: [c_uchar; 32usize],
	pub session: br_ssl_session_parameters,
	pub ecdhe_curve: c_uchar,
	pub ecdhe_point: [c_uchar; 133usize],
	pub ecdhe_point_len: c_uchar,
	pub reneg: c_uchar,
	pub saved_finished: [c_uchar; 24usize],
	pub flags: uint32_t,
	pub cpu: br_ssl_engine_context_AnonymousStruct_cpu,
	pub dp_stack: [uint32_t; 32usize],
	pub rp_stack: [uint32_t; 32usize],
	pub pad: [c_uchar; 512usize],
	pub hbuf_in: *mut c_uchar,
	pub hbuf_out: *mut c_uchar,
	pub saved_hbuf_out: *mut c_uchar,
	pub hlen_in: size_t,
	pub hlen_out: size_t,
	pub hsrun: Option<unsafe extern "C" fn(ctx: *mut c_void)>,
	pub action: c_uchar,
	pub alert: c_uchar,
	pub close_received: c_uchar,
	pub mhash: br_multihash_context,
	pub x509ctx: *mut *const br_x509_class,
	pub chain: *const br_x509_certificate,
	pub chain_len: size_t,
	pub cert_cur: *const c_uchar,
	pub cert_len: size_t,
	pub protocol_names: *mut *const c_char,
	pub protocol_names_num: uint16_t,
	pub selected_protocol: uint16_t,
	pub prf10: br_tls_prf_impl,
	pub prf_sha256: br_tls_prf_impl,
	pub prf_sha384: br_tls_prf_impl,
	pub iaes_cbcenc: *const br_block_cbcenc_class,
	pub iaes_cbcdec: *const br_block_cbcdec_class,
	pub iaes_ctr: *const br_block_ctr_class,
	pub ides_cbcenc: *const br_block_cbcenc_class,
	pub ides_cbcdec: *const br_block_cbcdec_class,
	pub ighash: br_ghash,
	pub ichacha: br_chacha20_run,
	pub ipoly: br_poly1305_run,
	pub icbc_in: *const br_sslrec_in_cbc_class,
	pub icbc_out: *const br_sslrec_out_cbc_class,
	pub igcm_in: *const br_sslrec_in_gcm_class,
	pub igcm_out: *const br_sslrec_out_gcm_class,
	pub ichapol_in: *const br_sslrec_in_chapol_class,
	pub ichapol_out: *const br_sslrec_out_chapol_class,
	pub iec: *const br_ec_impl,
	pub irsavrfy: br_rsa_pkcs1_vrfy,
	pub iecdsa: br_ecdsa_vrfy,
}

impl Clone for br_ssl_engine_context
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for br_ssl_engine_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

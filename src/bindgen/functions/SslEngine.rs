// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_ssl_engine_close(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_current_state(cc: *const br_ssl_engine_context) -> c_uint;
	pub fn br_ssl_engine_flush(cc: *mut br_ssl_engine_context, force: c_int);
	pub fn br_ssl_engine_inject_entropy(cc: *mut br_ssl_engine_context, data: *const c_void, len: size_t);
	pub fn br_ssl_engine_recvapp_ack(cc: *mut br_ssl_engine_context, len: size_t);
	pub fn br_ssl_engine_recvapp_buf(cc: *const br_ssl_engine_context, len: *mut size_t) -> *mut c_uchar;
	pub fn br_ssl_engine_recvrec_ack(cc: *mut br_ssl_engine_context, len: size_t);
	pub fn br_ssl_engine_recvrec_buf(cc: *const br_ssl_engine_context, len: *mut size_t) -> *mut c_uchar;
	pub fn br_ssl_engine_renegotiate(cc: *mut br_ssl_engine_context) -> c_int;
	pub fn br_ssl_engine_sendapp_ack(cc: *mut br_ssl_engine_context, len: size_t);
	pub fn br_ssl_engine_sendapp_buf(cc: *const br_ssl_engine_context, len: *mut size_t) -> *mut c_uchar;
	pub fn br_ssl_engine_sendrec_ack(cc: *mut br_ssl_engine_context, len: size_t);
	pub fn br_ssl_engine_sendrec_buf(cc: *const br_ssl_engine_context, len: *mut size_t) -> *mut c_uchar;
	pub fn br_ssl_engine_set_buffer(cc: *mut br_ssl_engine_context, iobuf: *mut c_void, iobuf_len: size_t, bidi: c_int);
	pub fn br_ssl_engine_set_buffers_bidi(cc: *mut br_ssl_engine_context, ibuf: *mut c_void, ibuf_len: size_t, obuf: *mut c_void, obuf_len: size_t);
	pub fn br_ssl_engine_set_default_aes_cbc(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_default_aes_gcm(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_default_chapol(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_default_des_cbc(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_default_ec(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_default_ecdsa(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_default_rsavrfy(cc: *mut br_ssl_engine_context);
	pub fn br_ssl_engine_set_suites(cc: *mut br_ssl_engine_context, suites: *const uint16_t, suites_num: size_t);
}

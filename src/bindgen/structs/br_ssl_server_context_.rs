// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[allow(missing_debug_implementations)]
pub struct br_ssl_server_context_
{
	pub eng: br_ssl_engine_context,
	pub client_max_version: uint16_t,
	pub cache_vtable: *mut *const br_ssl_session_cache_class,
	pub client_suites: [br_suite_translated; 40usize],
	pub client_suites_num: c_uchar,
	pub hashes: uint32_t,
	pub curves: uint32_t,
	pub policy_vtable: *mut *const br_ssl_server_policy_class,
	pub sign_hash_id: uint16_t,
	pub chain_handler: br_ssl_server_context__AnonymousUnion_chain_handler,
	pub ecdhe_key: [c_uchar; 70usize],
	pub ecdhe_key_len: size_t,
	pub ta_names: *const br_x500_name,
	pub tas: *const br_x509_trust_anchor,
	pub num_tas: size_t,
	pub cur_dn_index: size_t,
	pub cur_dn: *const c_uchar,
	pub cur_dn_len: size_t,
	pub hash_CV: [c_uchar; 64usize],
	pub hash_CV_len: size_t,
	pub hash_CV_id: c_int,
}


impl Default for br_ssl_server_context_
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

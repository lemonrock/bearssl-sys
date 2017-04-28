// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub struct br_ssl_client_context_
{
	pub eng: br_ssl_engine_context,
	pub min_clienthello_len: u16,
	pub hashes: u32,
	pub server_curve: c_int,
	pub client_auth_vtable: *mut *const br_ssl_client_certificate_class,
	pub auth_type: c_uchar,
	pub hash_id: c_uchar,
	pub client_auth: br_ssl_client_context___bindgen_ty_1,
	pub irsapub: br_rsa_public,
}

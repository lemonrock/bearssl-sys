// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_x509_knownkey_init_ec(ctx: *mut br_x509_knownkey_context, pk: *const br_ec_public_key, usages: c_uint);
	pub fn br_x509_knownkey_init_rsa(ctx: *mut br_x509_knownkey_context, pk: *const br_rsa_public_key, usages: c_uint);
}

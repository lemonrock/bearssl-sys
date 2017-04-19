// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	pub fn br_x509_minimal_init(ctx: *mut br_x509_minimal_context, dn_hash_impl: *const br_hash_class, trust_anchors: *const br_x509_trust_anchor, trust_anchors_num: size_t);
	pub fn br_x509_minimal_init_full(ctx: *mut br_x509_minimal_context, trust_anchors: *const br_x509_trust_anchor, trust_anchors_num: size_t);
}

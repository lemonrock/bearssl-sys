// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


pub type br_ecdsa_vrfy = Option<unsafe extern "C" fn(impl_: *const br_ec_impl, hash: *const c_void, hash_len: size_t, pk: *const br_ec_public_key, sig: *const c_void, sig_len: size_t) -> uint32_t>;

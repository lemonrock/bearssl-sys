// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


pub type br_ecdsa_sign = Option<unsafe extern "C" fn(impl_: *const br_ec_impl, hf: *const br_hash_class, hash_value: *const c_void, sk: *const br_ec_private_key, sig: *mut c_void) -> size_t>;

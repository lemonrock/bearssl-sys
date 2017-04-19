// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


pub type br_rsa_pkcs1_vrfy = Option<unsafe extern "C" fn(x: *const c_uchar, xlen: size_t, hash_oid: *const c_uchar, hash_len: size_t, pk: *const br_rsa_public_key, hash_out: *mut c_uchar) -> uint32_t>;

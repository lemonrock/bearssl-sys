// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


pub type br_poly1305_run = Option<unsafe extern "C" fn(key: *const c_void, iv: *const c_void, data: *mut c_void, len: usize, aad: *const c_void, aad_len: usize, tag: *mut c_void, ichacha: br_chacha20_run, encrypt: c_int)>;

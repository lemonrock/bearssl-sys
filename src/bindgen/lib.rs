// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.



use ::core::clone::Clone;
use ::core::default::Default;
use ::core::mem::zeroed;
use ::core::option::Option;

extern crate libc;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::int16_t;
use ::libc::size_t;
use ::libc::uint16_t;
use ::libc::uint32_t;
use ::libc::uint64_t;

#[link(name = "bearssl", kind = "static-nobundle")]
extern "C"
{
}

include!("bindgen/constants.rs");
include!("bindgen/types.rs");
include!("bindgen/structs.rs");
include!("bindgen/statics.rs");
include!("bindgen/functions.rs");

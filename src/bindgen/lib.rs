// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.



use ::core::clone::Clone;
use ::core::default::Default;
use ::core::mem::zeroed;
use ::core::option::Option;

#[link(name = "bearssl", kind = "static")]
extern "C"
{
}

include!("bindgen/constants.rs");
include!("bindgen/types.rs");
include!("bindgen/structs.rs");
include!("bindgen/statics.rs");
include!("bindgen/functions.rs");

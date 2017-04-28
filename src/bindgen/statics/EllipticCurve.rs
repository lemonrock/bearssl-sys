// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	#[link_name = "br_ec_all_m15"] pub static br_ec_all_m15: br_ec_impl;
	#[link_name = "br_ec_all_m31"] pub static br_ec_all_m31: br_ec_impl;
	#[link_name = "br_ec_c25519_i15"] pub static br_ec_c25519_i15: br_ec_impl;
	#[link_name = "br_ec_c25519_i31"] pub static br_ec_c25519_i31: br_ec_impl;
	#[link_name = "br_ec_c25519_m15"] pub static br_ec_c25519_m15: br_ec_impl;
	#[link_name = "br_ec_c25519_m31"] pub static br_ec_c25519_m31: br_ec_impl;
	#[link_name = "br_ec_p256_m15"] pub static br_ec_p256_m15: br_ec_impl;
	#[link_name = "br_ec_p256_m31"] pub static br_ec_p256_m31: br_ec_impl;
	#[link_name = "br_ec_prime_i15"] pub static br_ec_prime_i15: br_ec_impl;
	#[link_name = "br_ec_prime_i31"] pub static br_ec_prime_i31: br_ec_impl;
}

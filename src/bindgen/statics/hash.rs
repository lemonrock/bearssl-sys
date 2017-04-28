// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	#[link_name = "br_md5_vtable"] pub static br_md5_vtable: br_hash_class;
	#[link_name = "br_md5sha1_vtable"] pub static br_md5sha1_vtable: br_hash_class;
	#[link_name = "br_sha1_vtable"] pub static br_sha1_vtable: br_hash_class;
	#[link_name = "br_sha224_vtable"] pub static br_sha224_vtable: br_hash_class;
	#[link_name = "br_sha256_vtable"] pub static br_sha256_vtable: br_hash_class;
	#[link_name = "br_sha384_vtable"] pub static br_sha384_vtable: br_hash_class;
	#[link_name = "br_sha512_vtable"] pub static br_sha512_vtable: br_hash_class;
}

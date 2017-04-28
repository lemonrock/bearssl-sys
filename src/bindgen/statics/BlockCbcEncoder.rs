// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	#[link_name = "br_aes_big_cbcenc_vtable"] pub static br_aes_big_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_aes_ct64_cbcenc_vtable"] pub static br_aes_ct64_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_aes_ct_cbcenc_vtable"] pub static br_aes_ct_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_aes_pwr8_cbcenc_vtable"] pub static br_aes_pwr8_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_aes_small_cbcenc_vtable"] pub static br_aes_small_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_aes_x86ni_cbcenc_vtable"] pub static br_aes_x86ni_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_des_ct_cbcenc_vtable"] pub static br_des_ct_cbcenc_vtable: br_block_cbcenc_class;
	#[link_name = "br_des_tab_cbcenc_vtable"] pub static br_des_tab_cbcenc_vtable: br_block_cbcenc_class;
}

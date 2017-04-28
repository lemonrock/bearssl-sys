// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


extern "C"
{
	#[link_name = "br_sslrec_out_cbc_vtable"] pub static br_sslrec_out_cbc_vtable: br_sslrec_out_cbc_class;
	#[link_name = "br_sslrec_out_chapol_vtable"] pub static br_sslrec_out_chapol_vtable: br_sslrec_out_chapol_class;
	#[link_name = "br_sslrec_out_clear_vtable"] pub static br_sslrec_out_clear_vtable: br_sslrec_out_class;
	#[link_name = "br_sslrec_out_gcm_vtable"] pub static br_sslrec_out_gcm_vtable: br_sslrec_out_gcm_class;
}

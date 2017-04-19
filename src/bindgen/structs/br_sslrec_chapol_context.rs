// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct br_sslrec_chapol_context
{
	pub vtable: br_sslrec_chapol_context_AnonymousUnion_vtable,
	pub seq: uint64_t,
	pub key: [c_uchar; 32usize],
	pub iv: [c_uchar; 12usize],
	pub ichacha: br_chacha20_run,
	pub ipoly: br_poly1305_run,
}

impl Default for br_sslrec_chapol_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

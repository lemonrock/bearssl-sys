// This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


#[repr(C)]
pub struct __BindgenUnionField<T>(PhantomData<T>);

impl<T> __BindgenUnionField<T>
{
	#[inline(always)]
	pub fn new() -> Self
	{
		__BindgenUnionField(PhantomData)
	}
	#[inline(always)]
	pub unsafe fn as_ref(&self) -> &T
	{
		transmute(self)
	}
	#[inline(always)]
	pub unsafe fn as_mut(&mut self) -> &mut T
	{
		transmute(self)
	}
}

impl<T> Default for __BindgenUnionField<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl<T> Clone for __BindgenUnionField<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new()
	}
}

impl<T> Copy for __BindgenUnionField<T>
{
}

impl<T> Debug for __BindgenUnionField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> Result
	{
		fmt.write_str("__BindgenUnionField")
	}
}

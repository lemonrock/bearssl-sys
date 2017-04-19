# This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


include conf/Unix.mk

CC = x86_64-linux-musl-cc
CFLAGS = -O3 -ffunction-sections -fdata-sections
LD = x86_64-linux-musl-cc
AR = x86_64-linux-musl-ar

DLL = no
TOOLS = no
TESTS = no

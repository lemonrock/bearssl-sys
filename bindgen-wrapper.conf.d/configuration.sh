# This file is part of bearssl-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT. No part of bearssl-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of bearssl-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/bearssl-sys/master/COPYRIGHT.


bindingsName='bearssl'
rootIncludeFileName='bearssl.h'
macosXHomebrewPackageNames=''
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
headersFolderPath="$homeFolder"/compile-bearssl.conf.d/temporary/inc
libFolderPath="$homeFolder"/compile-bearssl.conf.d/temporary/build
link='bearssl'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	# Remove #[derive(Copy, Clone)] because of rust limitations with arrays
	local struct
	for struct in \
		br_des_gen_cbcdec_keys \
		br_des_gen_cbcenc_keys \
		br_ssl_engine_context_AnonymousUnion_in_ \
		br_ssl_engine_context_AnonymousUnion_out \
		br_sslrec_in_cbc_context \
		br_sslrec_in_cbc_context_AnonymousUnion_bc \
		br_sslrec_out_cbc_context \
		br_sslrec_out_cbc_context_AnonymousUnion_bc \
		br_ssl_client_context_
	do
		sed -i -e '/#\[derive(Copy, Clone)\]/d' "$outputFolderPath"/structs/"$struct".rs
	done

	# Remove #[derive(Copy)] because of rust limitations with arrays
	local struct
	for struct in \
		br_ssl_engine_context \
		br_ssl_server_context_
	do
		sed -i -e '/#\[derive(Copy)\]/d' "$outputFolderPath"/structs/"$struct".rs
	done
	
	# Remove Clone implementation for now because of changes in structs above
	local struct
	for struct in \
		br_ssl_engine_context \
		br_ssl_server_context_
	do
		sed -i -e '/impl Clone/,+7d' "$outputFolderPath"/structs/"$struct".rs
	done
}

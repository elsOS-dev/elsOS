#!/bin/bash

git clone git://git.savannah.gnu.org/grub.git
pushd grub
	./bootstrap
	./autogen.sh
	mkdir build
	pushd build
		../configure --disable-werror TARGET_CC=i686-elf-gcc TARGET_OBJCOPY=i686-elf-objcopy TARGET_STRIP=i686-elf-strip TARGET_NM=i686-elf-nm TARGET_RANLIB=i686-elf-ranlib --target=i686-elf --program-prefix=i386-pc-
		make -j$(nproc)
		sudo make install
	popd
popd

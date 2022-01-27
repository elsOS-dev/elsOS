arch ?= i686
kernel := build/kernel-$(arch).bin
iso := build/elsos-$(arch).iso
target ?= $(arch)-elsos
rust_os := target/$(target)/debug/libelsos.a

LD=$(HOME)/opt/cross/bin/i686-elf-ld
CC=$(HOME)/opt/cross/bin/i686-elf-gcc

CFLAGS=-m32 -std=gnu99 -ffreestanding -Wall -Wextra -c

export CFLAGS
export CC

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel

all: $(kernel)

run: $(iso)
	@qemu-system-x86_64 $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	@cp $(kernel) build/iso/boot/kernel.bin
	@cp $(grub_cfg) build/iso/boot/grub
	@grub-mkrescue -o $(iso) build/iso 2> /dev/null
	@rm -r build/iso

libc: libc.a

libc.a:
	@make -C src/libc
	@cp src/libc/libc.a .

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	$(LD) -n --gc-sections -T $(linker_script) 	\
		-o $(kernel) $(assembly_object_files) $(rust_os)

kernel: libc
	cargo +nightly build --target $(target).json

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf32 $< -o $@

clean:
	@rm -rf build
	@rm -f libc.a
	make -C src/libc clean

mrproper: clean
	cargo clean


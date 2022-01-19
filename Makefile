arch ?= i686
kernel := build/kernel-$(arch).bin
iso := build/elsos-$(arch).iso
target ?= $(arch)-elsos
rust_os := target/$(target)/debug/libelsos.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	@cp $(kernel) build/iso/boot/kernel.bin
	@cp $(grub_cfg) build/iso/boot/grub
	@grub-mkrescue -o $(iso) build/iso 2> /dev/null
	@rm -r build/iso

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@$(HOME)/opt/cross/bin/i686-elf-ld -n --gc-sections -T $(linker_script) 	\
		-o $(kernel) $(assembly_object_files) $(rust_os)

kernel:
	cargo +nightly build -Zbuild-std-features=compiler-builtins-mem --target $(target).json

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf32 $< -o $@


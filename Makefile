VERSION=0
PATCHLEVEL=0
SUBLEVEL=1
EXTRAVERSION=

export VERSION PATCHLEVEL SUBLEVEL EXTRAVERSION

DEBUG_RELEASE=debug
ARCH=i686
KERNEL=build/elsos-$(ARCH).bin
ISO=build/elsos-$(ARCH).iso
TARGET=$(ARCH)-elsos
RUST_OS=target/$(TARGET)/$(DEBUG_RELEASE)/libelsos.a

LD=$(ARCH)-elf-ld
CC=$(ARCH)-elf-gcc

CFLAGS=-m32 -std=gnu99 -ffreestanding -Wall -Wextra -c

export CFLAGS
export CC

LD_SCRIPT=src/arch/$(ARCH)/linker.ld
GRUB_CFG=grub/grub.cfg
ASM_SRC=$(wildcard src/arch/$(ARCH)/*.asm)
ASM_OBJ=$(subst src/, build/, ${ASM_SRC:.asm=.o})

.PHONY: all clean run iso kernel

all: $(KERNEL)

run: $(ISO)
	@qemu-system-x86_64 -drive format=raw,file=$(ISO) -serial stdio

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG)
	@mkdir -p build/iso/boot/grub
	@cp $(KERNEL) build/iso/boot/elsos.bin
	@cp $(GRUB_CFG) build/iso/boot/grub
	@grub-mkrescue -o $(ISO) build/iso 2> /dev/null
	@rm -r build/iso

libc: libc.a

libc.a:
	@make -C src/libc
	@cp src/libc/libc.a .

$(KERNEL): kernel $(RUST_OS) $(ASM_OBJ) $(LD_SCRIPT)
	$(LD) -n --gc-sections -T $(LD_SCRIPT) -o $(KERNEL) $(ASM_OBJ) $(RUST_OS)

kernel: libc
	cargo +nightly build --target $(TARGET).json

clippy: libc
	cargo +nightly clippy --target $(TARGET).json

# compile assembly files
build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf32 $< -o $@

clean:
	@rm -rf build
	@rm -f libc.a
	make -C src/libc clean

mrproper: clean
	cargo clean


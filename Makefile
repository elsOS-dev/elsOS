VERSION=0
PATCHLEVEL=0
SUBLEVEL=3
EXTRAVERSION=-$(shell git rev-parse HEAD | head -c 7)

export VERSION PATCHLEVEL SUBLEVEL EXTRAVERSION

DEBUG_RELEASE=debug
ARCH=i686
KERNEL=build/elsos-$(ARCH).bin
ISO=build/elsos-$(ARCH).iso
TARGET=$(ARCH)-elsos
RUST_KERNEL=target/$(TARGET)/$(DEBUG_RELEASE)/libelsos.a

LD=$(ARCH)-elf-ld
CC=$(ARCH)-elf-gcc
AR=$(ARCH)-elf-ar
QEMU=qemu-system-i386
QEMU_ARGS=-drive format=raw,file=$(ISO) -serial stdio
QEMU_MEMORY=-m 500M

CFLAGS=-m32 -std=gnu99 -ffreestanding -Wall -Wextra -c
ARFLAGS=rcs

export CFLAGS
export CC

LD_SCRIPT=linkers/$(ARCH).ld
GRUB_CFG=grub/grub.cfg

ASM_SRC=$(wildcard src/arch/$(ARCH)/*.asm)
ASM_OBJ=$(subst src/, build/, ${ASM_SRC:.asm=.o})

LIBC_SRC=$(wildcard src/libc/*.c)
LIBC_OBJ=$(subst src/, build/, ${LIBC_SRC:.c=.o})
LIBC_A=build/libc/libc.a

.PHONY: all clean run iso kernel libc

all: $(KERNEL)

run: $(ISO)
	$(QEMU) $(QEMU_ARGS) $(QEMU_MEMORY)

rund: $(ISO)
	$(QEMU) $(QEMU_ARGS) $(QEMU_MEMORY) -d int

rundd: $(ISO)
	$(QEMU) $(QEMU_ARGS) $(QEMU_MEMORY) -d int -s -S

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG)
	mkdir -p build/iso/boot/grub
	cp $(KERNEL) build/iso/boot/elsos.bin
	cp $(GRUB_CFG) build/iso/boot/grub
	grub-mkrescue -o $(ISO) build/iso 2> /dev/null
	rm -r build/iso

libc: $(LIBC_A)

$(LIBC_A): $(LIBC_OBJ)
	$(AR) $(ARFLAGS) $@ $(LIBC_OBJ)

$(KERNEL): $(ASM_OBJ) $(RUST_KERNEL) $(LD_SCRIPT)
	$(LD) -n --gc-sections -T $(LD_SCRIPT) -o $(KERNEL) $(ASM_OBJ) $(RUST_KERNEL)

$(RUST_KERNEL): libc
	cargo +nightly build --target $(TARGET).json

kernel: $(RUST_KERNEL)

clippy: libc
	cargo +nightly clippy --target $(TARGET).json

build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -f elf32 $< -o $@

build/libc/%.o: src/libc/%.c
	@mkdir -p $(shell dirname $@)
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -rf build

mrproper: clean
	cargo clean


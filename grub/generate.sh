#!/bin/bash
cd "$(dirname "$0")"

GRUB_DEFAULT=0
GRUB_TIMEOUT=0
if [ "$ELSOS_KBD_LAYOUT" = "qwerty" ]
then
	GRUB_DEFAULT=1
fi

if [ -z ${ELSOS_BOOT_TIMEOUT+x} ]
then
	GRUB_TIMEOUT=0
else
	GRUB_TIMEOUT=$ELSOS_BOOT_TIMEOUT
fi

cat > grub.cfg << EOF
default=$GRUB_DEFAULT
timeout=$GRUB_TIMEOUT

menuentry "elsOS with serial, azerty" {
   multiboot2 /boot/elsos.bin serial
   boot
}

menuentry "elsOS with serial, qwerty ansi" {
   multiboot2 /boot/elsos.bin serial qwerty
   boot
}

menuentry "elsOS, azerty" {
   multiboot2 /boot/elsos.bin
   boot
}

menuentry "elsOS, qwerty ansi" {
   multiboot2 /boot/elsos.bin qwerty
   boot
}
EOF

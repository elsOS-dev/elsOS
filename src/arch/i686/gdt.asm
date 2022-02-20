global _gdt_flush
extern _gp

_gdt_flush:
	lgdt [_gp]

	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	jmp 0x08:complete_flush

complete_flush:
	ret

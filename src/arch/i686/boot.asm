global start

section .bss
align 16
stack_bottom:
resb 16384 ; 16 KiB
stack_top:

section .text
global _start:function (_start.end - _start)
bits 32
_start:
	mov esp, stack_top

	extern kernel_main
	call kernel_main

	; print `OK` to screen
	mov dword [0xb8000], 0x2f4b2f4f

.hang:
	hlt
	jmp .hang
.end:

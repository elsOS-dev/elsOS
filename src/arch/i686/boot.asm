global start

section .bss
align 16
stack_bottom:
resb 16384
stack_top:

section .text
global _start:function (_start.end - _start)
bits 32
_start:
	cli
	mov esp, stack_top
	xor ebp, ebp

	push ebx
	push eax

	extern kernel_main
	call kernel_main

.hang:
	cli
	hlt
	jmp .hang
.end:

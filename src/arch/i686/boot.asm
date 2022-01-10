global start

section .text
bits 32
start:
	extern kernel_main
	call kernel_main
	; print `OK` to screen
	mov dword [0xb8000], 0x2f4b2f4f
	hlt


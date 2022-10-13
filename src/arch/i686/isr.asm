;macro to define isr_error
%macro isr_error 1
isr%+%1:
	push %1
	jmp isr_common
%endmacro

;macro to define isr_no_error
%macro isr_no_error 1
isr%+%1:
	; push a fake error code to have the same structure model
	push 0
	push %1
	jmp isr_common
%endmacro

;define all isr handlers
isr_no_error 0
isr_no_error 1
isr_no_error 2
isr_no_error 3
isr_no_error 4
isr_no_error 5
isr_no_error 6
isr_no_error 7
isr_no_error 8
isr_no_error 9
isr_error    10
isr_error    11
isr_error    12
isr_error    13
isr_error    14
isr_no_error 15
isr_no_error 16
isr_error    17
isr_no_error 18
isr_no_error 19
isr_no_error 20
isr_no_error 21
isr_no_error 22
isr_no_error 23
isr_no_error 24
isr_no_error 25
isr_no_error 26
isr_no_error 27
isr_no_error 28
isr_no_error 29
isr_error    30
isr_no_error 31

isr_no_error 32

isr_common:
	cli
	; push eax, ecx, edx, ebx, esi, edi
	pushad
	cld
;
;	; save ds in eax then push it
	xor eax, eax
	mov ax, ds
	push eax

	; use kernel_data segment
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	push esp ; pass the saved args to the function via the old stack pointer
	extern interrupt_handler
	call interrupt_handler
	add esp, 4

	pop eax

	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	popad

	; remove error code and interrupt number
	add esp, 8

	iret

; define an isr_table to get easily pointers from rust code
global _isr_table
_isr_table:
%assign i 0
%rep 33
	dd isr%+i
%assign i i+1
%endrep

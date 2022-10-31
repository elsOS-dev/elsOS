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
; exceptions
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

; irqs
isr_no_error 32
isr_no_error 33
isr_no_error 34
isr_no_error 35
isr_no_error 36
isr_no_error 37
isr_no_error 38
isr_no_error 39
isr_no_error 40
isr_no_error 41
isr_no_error 42
isr_no_error 43
isr_no_error 44
isr_no_error 45
isr_no_error 46
isr_no_error 47

; other interrupts
isr_no_error 48
isr_no_error 49
isr_no_error 50
isr_no_error 51
isr_no_error 52
isr_no_error 53
isr_no_error 54
isr_no_error 55
isr_no_error 56
isr_no_error 57
isr_no_error 58
isr_no_error 59
isr_no_error 60
isr_no_error 61
isr_no_error 62
isr_no_error 63
isr_no_error 64
isr_no_error 65
isr_no_error 66
isr_no_error 67
isr_no_error 68
isr_no_error 69
isr_no_error 70
isr_no_error 71
isr_no_error 72
isr_no_error 73
isr_no_error 74
isr_no_error 75
isr_no_error 76
isr_no_error 77
isr_no_error 78
isr_no_error 79
isr_no_error 80
isr_no_error 81
isr_no_error 82
isr_no_error 83
isr_no_error 84
isr_no_error 85
isr_no_error 86
isr_no_error 87
isr_no_error 88
isr_no_error 89
isr_no_error 90
isr_no_error 91
isr_no_error 92
isr_no_error 93
isr_no_error 94
isr_no_error 95
isr_no_error 96
isr_no_error 97
isr_no_error 98
isr_no_error 99
isr_no_error 100
isr_no_error 101
isr_no_error 102
isr_no_error 103
isr_no_error 104
isr_no_error 105
isr_no_error 106
isr_no_error 107
isr_no_error 108
isr_no_error 109
isr_no_error 110
isr_no_error 111
isr_no_error 112
isr_no_error 113
isr_no_error 114
isr_no_error 115
isr_no_error 116
isr_no_error 117
isr_no_error 118
isr_no_error 119
isr_no_error 120
isr_no_error 121
isr_no_error 122
isr_no_error 123
isr_no_error 124
isr_no_error 125
isr_no_error 126
isr_no_error 127
isr_no_error 128
isr_no_error 129
isr_no_error 130
isr_no_error 131
isr_no_error 132
isr_no_error 133
isr_no_error 134
isr_no_error 135
isr_no_error 136
isr_no_error 137
isr_no_error 138
isr_no_error 139
isr_no_error 140
isr_no_error 141
isr_no_error 142
isr_no_error 143
isr_no_error 144
isr_no_error 145
isr_no_error 146
isr_no_error 147
isr_no_error 148
isr_no_error 149
isr_no_error 150
isr_no_error 151
isr_no_error 152
isr_no_error 153
isr_no_error 154
isr_no_error 155
isr_no_error 156
isr_no_error 157
isr_no_error 158
isr_no_error 159
isr_no_error 160
isr_no_error 161
isr_no_error 162
isr_no_error 163
isr_no_error 164
isr_no_error 165
isr_no_error 166
isr_no_error 167
isr_no_error 168
isr_no_error 169
isr_no_error 170
isr_no_error 171
isr_no_error 172
isr_no_error 173
isr_no_error 174
isr_no_error 175
isr_no_error 176
isr_no_error 177
isr_no_error 178
isr_no_error 179
isr_no_error 180
isr_no_error 181
isr_no_error 182
isr_no_error 183
isr_no_error 184
isr_no_error 185
isr_no_error 186
isr_no_error 187
isr_no_error 188
isr_no_error 189
isr_no_error 190
isr_no_error 191
isr_no_error 192
isr_no_error 193
isr_no_error 194
isr_no_error 195
isr_no_error 196
isr_no_error 197
isr_no_error 198
isr_no_error 199
isr_no_error 200
isr_no_error 201
isr_no_error 202
isr_no_error 203
isr_no_error 204
isr_no_error 205
isr_no_error 206
isr_no_error 207
isr_no_error 208
isr_no_error 209
isr_no_error 210
isr_no_error 211
isr_no_error 212
isr_no_error 213
isr_no_error 214
isr_no_error 215
isr_no_error 216
isr_no_error 217
isr_no_error 218
isr_no_error 219
isr_no_error 220
isr_no_error 221
isr_no_error 222
isr_no_error 223
isr_no_error 224
isr_no_error 225
isr_no_error 226
isr_no_error 227
isr_no_error 228
isr_no_error 229
isr_no_error 230
isr_no_error 231
isr_no_error 232
isr_no_error 233
isr_no_error 234
isr_no_error 235
isr_no_error 236
isr_no_error 237
isr_no_error 238
isr_no_error 239
isr_no_error 240
isr_no_error 241
isr_no_error 242
isr_no_error 243
isr_no_error 244
isr_no_error 245
isr_no_error 246
isr_no_error 247
isr_no_error 248
isr_no_error 249
isr_no_error 250
isr_no_error 251
isr_no_error 252
isr_no_error 253
isr_no_error 254
isr_no_error 255

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
%rep 256
	dd isr%+i
%assign i i+1
%endrep

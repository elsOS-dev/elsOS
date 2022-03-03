ENTRY(_start)

SECTIONS
{
    . = 0x00100000;
    .boot BLOCK(4K) : ALIGN(4K)
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))
    }
    .text BLOCK(4K) : ALIGN(4K)
    {
        _kernel_start = .;
        *(.text)
    }
    /* Read-only data. */
    .rodata BLOCK(4K) : ALIGN(4K)
    {
        *(.rodata)
    }
 
    /* Read-write data (initialized) */
    .data BLOCK(4K) : ALIGN(4K)
    {
        *(.data)
    }
    .bss BLOCK(4K) : ALIGN(4K)
    {
        *(COMMON)
        *(.bss)
    }
    _kernel_end = .;
}

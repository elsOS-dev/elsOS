ENTRY(_start)

SECTIONS
{
    . = 0x100000;
    _kernel_start = .;
    .boot ALIGN(4K) :
    {
        KEEP(*(.multiboot_header))
    }
    .text ALIGN(4K) :
    {
        *(.text)
    }
    /* Read-only data. */
    .rodata ALIGN(4K) :
    {
        *(.rodata)
    }
    /* Read-write data (initialized) */
    .data ALIGN(4K) :
    {
        *(.data)
    }
    .bss ALIGN(4K) :
    {
        *(COMMON)
        *(.bss)
    }
    .stab ALIGN(4K) :
    {
        *(.stab)
    }
    .stabstr ALIGN(4K) :
    {
        *(.stabstr)
    }
    .note ALIGN(4K) :
    {
        *(.note)
    }
    .comment ALIGN(4K) :
    {
        *(.comment)
    }
    _kernel_end = .;
}

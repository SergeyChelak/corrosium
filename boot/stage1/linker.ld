ENTRY(_start)

SECTIONS {
    . = 0x500;
    _stack_start = .;

    . = 0x7c00;
    _stack_end = .; 

    .boot :
    {
        *(.boot .boot.*)
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }
    
    .data :
    {
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.got .got.*)
    }

   . = 0x7c00 + 0x1fe;

    .magic_number :
    {
        SHORT(0xaa55)
    }

    _next_stage = .;
}
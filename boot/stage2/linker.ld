ENTRY(_stage2)

SECTIONS {
    /* second sector */
    . = 0x7e00;

    .start : {
        *(.start)
    }

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }

    . = 0x7e00 + 63 * 512;
}
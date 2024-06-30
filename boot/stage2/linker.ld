ENTRY(_stage2)

SECTIONS {
    /* second sector */
    . = 0x7e00;

    .start : {
        *(.start)
    }

    .text : ALIGN(4096) {
        *(.text .text.*)
    }

    .bss : ALIGN(4096) {
        *(.bss .bss.*)
    }

    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }
    
    .data : ALIGN(4096) {
        *(.data .data.*)
    }
}
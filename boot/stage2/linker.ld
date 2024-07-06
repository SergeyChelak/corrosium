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

    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }
    
    .data : ALIGN(4096) {
        *(.data .data.*)
    }

    .bss : ALIGN(4096) {
        *(.bss .bss.*)
    }

    _disk_buffer = .;
    . = _disk_buffer + 512;
    _fat_table = .;
    . = _fat_table + 20 * 512;

    . = 0x7e00 + 63 * 512;
}
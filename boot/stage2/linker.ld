ENTRY(_stage2)

SECTIONS {
    /* second sector */
    . = 0x7c00 + 512;

    .start : {
        *(.start)
    }

    /* code section */
    .text : {
        *(.text .text.*)
    }

    /* static variables section */
    .bss : {
        *(.bss .bss.*)
    }

    /* read-only data, like consts */
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }

    /* contains info about exceptions */
    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }
}
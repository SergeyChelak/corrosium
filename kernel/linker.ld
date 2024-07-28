ENTRY(kernel_main)

SECTIONS {
    . = 0x100000;

    /* code section */
    .text : AT(0x100000) {
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
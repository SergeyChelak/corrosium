ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_OFFSET = 0xFFFFFFFF80000000;

SECTIONS {
    . = KERNEL_OFFSET;

    . += SIZEOF_HEADERS;

    .text ALIGN(4K) : AT(ADDR(.text) - KERNEL_OFFSET) {
        *(.text*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        *(.rodata*)
    }

    .data ALIGN(4K) : AT(ADDR(.data) - KERNEL_OFFSET) {
        *(.data*)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_OFFSET) {
        *(.bss*)
    }

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}

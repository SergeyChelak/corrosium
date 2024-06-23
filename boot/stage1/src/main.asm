.section .boot, "awx"
.global _start
.code16

_start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax
    mov sp, _start
    cld
    call _stage1

spin:
    cli
    hlt
    jmp spin
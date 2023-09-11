;*********************************************************************************
; Corrosium OS Bootloader
;*********************************************************************************

;----------------------------------------------------------------
; Stage 1: 
;    Setup stack
;    Load main part of the bootloader into the RAM
;----------------------------------------------------------------

bits 16

org 0x7c00
    jmp short stage1_entrypoint     ; Skip BIOS parameter block
    nop
    times 33 db 0                   

stage1_entrypoint:                  ; Some BIOS may load us at 0x0000:0x7C00 while others at 0x07C0:0x0000
cli                             ; Clear interruptions
    jmp 0x0000:.setup_segments      ; Do a far jump to accommodate for this issue (CS is reloaded to 0x0000)
    .setup_segments:                ; Set all segment registers to zero
        xor ax, ax
        mov ss, ax
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov sp, stage1_entrypoint   ; Set up a stack that it starts growing below stage1_entrypoint (0x0000:0x7c00)
    sti                             ; Enable interruptions

   call Bios_test_disk_service
; load the rest of the loader...
    mov ax, 1                                       ; ax: start sector
    mov cx, (stage2_end - stage2_entrypoint) / 512  ; cx: number of sectors (512 bytes) to read
    mov bx, stage2_entrypoint   ; bx: offset of buffer
    xor dx, dx                  ; dx: segment of buffer
    call Bios_read_disk

    mov dl, disk_id
    jmp stage2_entrypoint       ; stage2_entrypoint should be 0x7e00
; ... and jump to it

%include "src/print.asm"
%include "src/disk.asm"

times 510-($-$$) db 0               ; Padding
dw 0xAA55                           ; Boot signature

;----------------------------------------------------------------
; Stage 2
;   Enable A20 line
;   Enable paging
;   Setup Programmable Interrupt Controller
;   Enter long mode
;   Load kernel and jump to it
;----------------------------------------------------------------
stage2_entrypoint:
    mov si, msg_stage1_success
    call Bios_print

    call Check_long_mode_support

    call Enable_a20

    ; load GDT and enable protected mode
    cli
    lgdt [gdt.descriptor]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp CODE_SEG:protected_mode

%include "src/long_mode.asm"
%include "src/a20.asm"

msg_stage1_success          db 'Stage 1 succeeded', 13, 10, 0
CODE_SEG equ gdt.code - gdt
; DATA_SEG equ gdt.data - gdt
gdt:
    .null:
        dd 0
        dd 0
    .code:
        dw 0xffff
        dw 0
        db 0
        db 0x9a
        db 11001111b
        db 0
    .data:
        dw 0xffff
        dw 0
        db 0
        db 0x92
        db 11001111b
        db 0    
    .descriptor:
        dw $ - gdt - 1         ; 16-bit Size (Limit) of GDT
        dd gdt                 ; 32-bit Base Address of GDT

[bits 32]
protected_mode:
    .hlt: hlt
    jmp .hlt

   align 512, db 0
stage2_end:
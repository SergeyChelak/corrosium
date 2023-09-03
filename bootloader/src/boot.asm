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

;----------------------------------------------------------------
    call BIOS_test_disk_service
stage1_load_loader:
    mov ax, 1                   ; ax: start sector
    mov cx, 1                   ; cx: number of sectors (512 bytes) to read
    mov bx, stage2_entrypoint   ; bx: offset of buffer
    xor dx, dx                  ; dx: segment of buffer
    call BIOS_read_disk

    mov dl, disk_id
    jmp stage2_entrypoint       ; stage2_entrypoint should be 0x7e00
;----------------------------------------------------------------

%include "src/print.asm"
%include "src/disk.asm"

times 510-($-$$) db 0               ; Padding
dw 0xAA55                           ; Boot signature

;----------------------------------------------------------------
; Stage 2
;----------------------------------------------------------------
stage2_entrypoint:
    mov si, stage1_success_message
    call BIOS_print
    .hlt:
        hlt
        jmp .hlt

stage1_success_message  db 'Stage 1 succeeded', 13, 10, 0
; align 512, db 0
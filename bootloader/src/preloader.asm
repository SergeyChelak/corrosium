;*********************************************************************************
; Corrosium OS Bootloader
; Loads rest of bootloader and kernel into the RAM
;*********************************************************************************

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

;----------------------------------------------------------------
; TODO: load stage 2 into the RAM
stage1_load_loader:
    mov dl, [stage1_disk_id]
    mov ax, 0x42
    int 0x13

;----------------------------------------------------------------
    mov si, stage1_success_message
    call BIOS_print

; TODO: should be jump to stage 2
;----------------------------------------------------------------

.halt:
    hlt
    jmp .halt

%include "src/bios.asm"

stage1_success_message  db 'Real mode stage succeeded', 13, 10, 0

times 510-($-$$) db 0               ; Padding
dw 0xAA55                           ; Boot signature

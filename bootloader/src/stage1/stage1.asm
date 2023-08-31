;----------------------------------------------------------------
; Corrosium OS
; Bootloader in real mode
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
stage1_test_disk_service:
    mov [stage1_disk_id], dl
    mov ah, 0x41
    mov bx, 0x55aa
    int 0x13
    jc stage1_disk_ext_not_supported
    cmp bx, 0xaa55
    jne stage1_disk_ext_not_supported

;----------------------------------------------------------------
; TODO: load stage 2 into the RAM

;----------------------------------------------------------------
    mov si, stage1_success_message
    call BIOS_print

; TODO: should be jump to stage 2
;----------------------------------------------------------------

stage1_halt:
    hlt
    jmp stage1_halt

stage1_disk_ext_not_supported:
    mov si, stage1_disk_ext_error_message
    jmp stage1_halt

%include "src/stage1/bios.asm"

stage1_disk_id db 0x0
stage1_success_message  db 'Real mode stage succeeded', 13, 10, 0
stage1_disk_ext_error_message  db 'Disk extension is not supported', 13, 10, 0

times 510-($-$$) db 0               ; Padding
dw 0xAA55                           ; Boot signature

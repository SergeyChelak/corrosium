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

;----------------------------------------------------------------
; TODO: load the rest of the loader
stage1_load_loader:
    mov si, disk_read_packet
    mov word[si], 16            ; size, 16 bytes
    mov word[si+2], 1           ; number sectors to read, 5 sectors
    mov word[si+4], 0x7e00      ; offset
    mov word[si+6], 0           ; segment
    mov word[si+8], 1           ; address low
    mov word[si+10], 0          ; address high
    mov dl, [stage1_disk_id]
    mov ah, 0x42
    int 0x13
    jc stage1_read_loader_error

    mov dl, stage1_disk_id
    jmp stage2_entrypoint       ; stage2_entrypoint should be 0x7e00
;----------------------------------------------------------------

stage1_halt:
    hlt
    jmp stage1_halt

stage1_read_loader_error:
    mov si, stage1_read_loader_error_message
    call BIOS_print
    jmp stage1_halt

%include "src/bios.asm"

stage1_tmp_message  db 'Processing to stage 2', 13, 10, 0
stage1_read_loader_error_message  db 'Failed to read a stage 2 from the disk', 13, 10, 0

; -- Disk read struct
disk_read_packet: times 16 db 0

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
; Corrosium OS
; Bootloader in real mode

bits 16
org 0x7c00
    jmp short stage1_entrypoint     ; Skip BIOS parameter block
    nop
    times 33 db 0                   

stage1_entrypoint:                  ; Some BIOS may load us at 0x0000:0x7C00 while others at 0x07C0:0x0000.
    cli                             ; Disable interruptions
    jmp 0x0000:.setup_segments      ; We do a far jump to accommodate for this issue (CS is reloaded to 0x0000).
    .setup_segments:                ; Next, we set all segment registers to zero.
        xor ax, ax
        mov ss, ax
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov sp, stage1_entrypoint   ; We set up a temporary stack so that it starts growing below stage1_entrypoint (i.e. the stack base will be located at 0:0x7c00).
        cld                         ; Clear the direction flag (i.e. go forward in memory when using instructions like lodsb).
    sti                             ; Enable interruptions

    mov si, stage1_message
    call BIOS_print

.loop:
    hlt
    jmp .loop

%include "src/stage1/bios.asm"

stage1_message  db 'Real mode stage completed', 13, 10, 0

times 510-($-$$) db 0               ; Padding
dw 0xAA55                           ; Boot signature

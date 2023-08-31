; Corrosium OS
; Bootloader in real mode

[bits 16]
[org 0x7c00]

times 90 db 0                       ; skip BIOS parameter block

stage1_entrypoint:                  ; Main entry point where BIOS leaves us. Some BIOS may load us at 0x0000:0x7C00 while others at 0x07C0:0x0000.
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
    call BIOS_println

.loop:
    hlt
    jmp .loop

%include "src/stage1/bios_print.asm"

stage1_message dw 13
db 'Real mode: OK'

times 510-($-$$) db 0               ; Padding
dw 0xAA55                           ; Boot signature

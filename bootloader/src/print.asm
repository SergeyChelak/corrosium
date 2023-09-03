;*********************************************************************************
; Corrosium OS
; BIOS text print
;*********************************************************************************
bits 16

BIOS_print:
;*********************************************************************************
; Prints a zero-terminated string via BIOS function
;---------------------------------------------------------------------------------
; si: pointer to string
;*********************************************************************************
    push ax
    push bx
    push si
    mov bx, 0
    .string_loop:
        lodsb
        cmp al, 0
        je .string_done
        mov ah, 0eh
        int 10h
    jmp .string_loop

    .string_done:
    pop si
    pop bx
    pop ax
    ret
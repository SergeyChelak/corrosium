;*********************************************************************************
; Corrosium OS
; BIOS utils
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

;*********************************************************************************
; Stores current drive identifier and checks if disk service is supported
; If service isn't supported, prints corresponding message and halts the CPU
;*********************************************************************************
BIOS_test_disk_service:
    mov [stage1_disk_id], dl
    mov ah, 0x41
    mov bx, 0x55aa
    int 0x13
    jc .BIOS_disk_service_not_supported
    cmp bx, 0xaa55
    jne .BIOS_disk_service_not_supported
    ret
    .BIOS_disk_service_not_supported:
        mov si, stage1_disk_ext_error_message
        .halt:
            hlt
            jmp .halt

stage1_disk_id db 0x0
stage1_disk_ext_error_message  db 'Disk extension is not supported', 13, 10, 0
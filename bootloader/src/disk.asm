;*********************************************************************************
; Corrosium OS
; BIOS disk utils
;*********************************************************************************

bits 16
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
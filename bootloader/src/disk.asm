;*********************************************************************************
; Corrosium OS
; BIOS disk utils
;*********************************************************************************

bits 16

disk_id                         db 0x0

msg_disk_service_unsupported    db 'Disk service is not supported', 13, 10, 0
msg_disk_read_error             db 'Failed to read from the disk', 13, 10, 0

DAP:
;*******************************************************************************;
; Disk Address Packet                                                           ;
;-------------------------------------------------------------------------------;
; Offset  Size   Description                                                    ;
;   0       1    size of packet (16 bytes)                                      ;
;   1       1    always 0                                                       ;
;   2       2    number of sectors to load (max = 127 on some BIOS)             ;
;   4       2    16-bit offset of target buffer                                 ;
;   4       2    16-bit segment of target buffer                                ;
;   8       4    lower 32 bits of 48-bit starting LBA                           ;
;  12       4    upper 32 bits of 48-bit starting LBA                           ;
;*******************************************************************************;
              db 0x10 ; size of packet = 16 bytes
              db 0    ; always 0
.num_sectors: dw 127  ; number of sectors to load (max = 127 on some BIOS)
.buf_offset:  dw 0x0  ; 16-bit offset of target buffer
.buf_segment: dw 0x0  ; 16-bit segment of target buffer
.LBA_lower:   dd 0x0  ; lower 32 bits of 48-bit starting LBA
.LBA_upper:   dd 0x0  ; upper 32 bits of 48-bit starting LBA

Bios_read_disk:
;**********************************************************;
; Load disk sectors to memory (int 13h, function code 42h) ;
;----------------------------------------------------------;
; ax: start sector                                         ;
; cx: number of sectors (512 bytes) to read                ;
; bx: offset of buffer                                     ;
; dx: segment of buffer                                    ;
;**********************************************************;
    .start:
        cmp cx, 127 ; (max sectors to read in one call = 127)
        jbe .good_size
         pusha
        mov cx, 127
        call Bios_read_disk
        popa
        add eax, 127
        add dx, 127 * 512 / 16
        sub cx, 127
        jmp .start

    .good_size:
        mov [DAP.LBA_lower], ax
        mov [DAP.num_sectors], cx
        mov [DAP.buf_segment], dx
        mov [DAP.buf_offset], bx
        mov dl, [disk_id]
        mov si, DAP
        mov ah, 0x42
        int 0x13
        jc .print_error
        ret
    .print_error:
        mov si, msg_disk_read_error
        call Bios_print
    .halt: hlt
     jmp .halt


;*********************************************************************************
; Stores current drive identifier and checks if disk service is supported
; If service isn't supported, prints corresponding message and halts the CPU
;*********************************************************************************
Bios_test_disk_service:
    mov [disk_id], dl
    mov ah, 0x41
    mov bx, 0x55aa
    int 0x13
    jc .not_supported
    cmp bx, 0xaa55
    jne .not_supported
    ret
    .not_supported:
        mov si, msg_disk_service_unsupported
        .halt: hlt
        jmp .halt

;*********************************************************************************
; Corrosium OS
; --------------------------------------------------------------------------------
; Functions to enable & check if enabled A20 line
; https://wiki.osdev.org/A20_Line
;*********************************************************************************

bits 16

msg_a20_enabled                 db 'Line A20 is enabled', 13, 10, 0
msg_failed_to_enable_a20        db 'Failed to enable line A20', 13, 10, 0

;*********************************************************************************
; Makes attempt to enable A20 line (in real mode)
; --------------------------------------------------------------------------------
; This OS isn't expected to be run on real hardware but with QEMU
; It means that A20 line is enabled and fast a20 get is added just it case
;*********************************************************************************
Enable_a20:
    call Check_a20
    test ax, ax
    jnz .end

    call Fast_a20
    
    call Check_a20
    test ax, ax
    jnz .end
    
    mov si, msg_failed_to_enable_a20
    call Bios_print
    .hlt: hlt
    
    jmp .hlt
    .end:
        mov si, msg_a20_enabled
        call Bios_print
        ret

;*********************************************************************************
; Check the status of the A20 line (in real mode)
; --------------------------------------------------------------------------------
; Returns: ax = 0 if the a20 line is disabled
;          ax = 1 if the a20 line is enabled
;*********************************************************************************
Check_a20:
    pushf
    push ds
    push es
    push di
    push si
    cli ; clear interrupts
    
    xor ax, ax ; ax = 0
    mov es, ax ; es = 0
    not ax     ; ax = 0xFFFF
    mov ds, ax ; ds = 0xFFFF
    mov di, 0x0500 ; 0500 and 0510 are chosen since they are guaranteed to be free 
    mov si, 0x0510 ; for use at any point of time after BIOS initialization.

    ; save the original values found at these addresses.
    mov dl, byte [es:di]  
    push dx
    mov dl, byte [ds:si]
    push dx
    
    mov byte [es:di], 0x00 ; [es:di] is 0:0500
    mov byte [ds:si], 0xFF ; [ds:si] is FFFF:0510 
    cmp byte [es:di], 0xFF ; if the A20 line is disabled, [es:di] will contain 0xFF
                           ; (as the write to [ds:si] really occured to 00500).

    mov ax, 0 ; A20 disabled ([es:di] equal to 0xFF).
    je .a20_disabled
    mov ax, 1 ; A20 enabled.
   .a20_disabled:

    ; restore original values
    pop dx  
    mov byte [ds:si], dl
    pop dx
    mov byte [es:di], dl
   
    pop si
    pop di
    pop es
    pop ds
    popf
    sti ; Enable interrupts.
    ret

;*********************************************************************************
; Enable A20 Line via IO port 92h (Fast A20 method)                   
;*********************************************************************************
Fast_a20:
    in al, 0x92
    test al, 2
    jnz .end
    or al, 2
    and al, 0xFE
    out 0x92, al
    .end: ret
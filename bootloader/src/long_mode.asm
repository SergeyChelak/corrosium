;*********************************************************************************
; Corrosium OS
; Long mode service functions
;*********************************************************************************

bits 16

;********************************************************************;
; Check if Long mode is supported                                    ;
;--------------------------------------------------------------------;
; Returns: eax = 0 if Long mode is NOT supported, else non-zero.     ;
;********************************************************************;
check_long_mode_support:
    mov eax, 0x80000000 ; Test if extended processor info in available.  
    cpuid                
    cmp eax, 0x80000001 
    jb .not_supported     

    mov eax, 0x80000001 ; After calling CPUID with EAX = 0x80000001, 
    cpuid               ; all AMD64 compliant processors have the longmode-capable-bit
    test edx, (1 << 29) ; (bit 29) turned on in the EDX (extended feature flags).

    jz .not_supported   ; If it's not set, there is no long mode.
    ret

   .not_supported:
      xor eax, eax
      ret
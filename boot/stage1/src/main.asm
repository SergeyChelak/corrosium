.section .boot, "awx"
.global _start
.code16

_start:
    jmp actual_start
    nop

bios_paramter_block:
    oem_id:
        .byte 'v', 'e', 'n', 'd', 'o', 'r', '8', '6'
    bytes_per_sector:
    	.word 0x200
    sectors_per_cluster:
        .byte 4
    reserved_sectors:
        .word 64
    number_of_fats:		
        .byte 2
    root_directory_entries:	
        .word 512
    total_sectors:
    	.word 20480
    media_type:			
        .byte 0xf8
    sectors_per_fat:
    	.word 20
    sectors_per_track:		
        .word 32
    number_of_heads:			
        .word 2
    hidden_sectors:		
        .4byte 0
    total_sectors_u32:	
        .4byte 0

extended_boot_record:
    drive:				
        .byte 0x80
    reverved:
    	.byte 0
    nt_signature:
        .byte 0x29
    volume_serial:		
        .4byte 0x57ab1e5
    disk_label:			
        .byte 'C', 'o', 'r', 'r', 'o', 's', 'i', 'u', 'm', 'O', 'S'
    fs_name:			
        .byte 'F', 'A', 'T', '1', '6', ' ', ' ', ' '

actual_start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax
    mov sp, _start
    cld
    call _stage1

spin:
    cli
    hlt
    jmp spin
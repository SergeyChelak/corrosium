use core::{arch::asm, ptr::addr_of};

/// Common FAT-family boot Sector and BIOS parameter blocks
#[repr(C, packed)]
pub struct FatHeader {
    pub jump_instruction: [u8; 3],
    pub oem_name: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors_count: u16,
    pub fat_count: u8,
    pub root_directory_entries: u16,
    pub total_sectors: u16,
    pub media_descriptor_type: u8,
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16, // for interrupt 0x13
    pub number_of_heads: u16,   // for interrupt 0x13
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,
}

pub fn load_header() -> FatHeader {
    let buffer = [0u8; 512];
    load(0, 1, addr_of!(buffer) as u32);
    let header: FatHeader = unsafe { core::ptr::read(buffer.as_ptr() as *const _) };
    header
}

// https://wiki.osdev.org/ATA_read/write_sectors
fn load(lba: u32, sectors: u8, target: u32) {
    unsafe {
        asm!(
            "mov ebx, eax",

            "mov edx, 0x01f6",      // port to send drive and 24-27 of LBA
            "shr eax, 24",
            "or al, 11100000b",     // select master drive
            "out dx, al",

            "mov edx, 0x01f2",      // port to send number of sectors
            "mov al, cl",
            "out dx, al",

            "mov edx, 0x1f3",       // port to send bit 0-7 of LBA
            "mov eax, ebx",
            "out dx, al",

            "mov edx, 0x1f4",       // port to send bit 8-15 of LBA
            "mov eax, ebx",
            "shr eax, 8",
            "out dx, al",

            "mov edx, 0x1f5",       // port to send bit 16-23 of LBA
            "mov eax, ebx",
            "shr eax, 16",
            "out dx, al",

            "mov edx, 0x1f7",       // command port
            "mov al, 0x20",         // read with retry
            "out dx, al",

            "2:",                   // still going
            "in al, dx",
            "test al, 8",           // sector buffer require servicing
            "jz 2b",                // until the sector buffer is ready

            "mov eax, 256",         // read 1 sector = 256 words
            "xor bx, bx",
            "mov bl, cl",           // read CL sectors
            "mul bx",
            "mov ecx, eax",         // rcx is counter for INSW
            "mov edx, 0x1f0",       // data port, in and out
            "rep insw",

            in("eax") lba,
            in("cl") sectors,
            in("edi") target,
        )
    }
}

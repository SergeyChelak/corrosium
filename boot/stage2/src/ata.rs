use core::arch::asm;

// https://wiki.osdev.org/ATA_read/write_sectors
pub fn load(lba: u32, sectors: u8, target: u32) {
    unsafe {
        asm!("pushfd", "push eax", "push ebx", "push ecx", "push edx", "push edi",);
        asm!(
            "mov ebx, eax",

            "mov edx, 0x01f6",      // port to send drive and 24-27 of LBA
            "shr eax, 24",
            "or al, 11100000b",     // select master drive
            "out dx, al",

            "mov edx, 0x01f2",      // port to send number of sectors
            "mov al, cl",
            "out dx, al",

            "mov edx, 0x01f3",       // port to send bit 0-7 of LBA
            "mov eax, ebx",
            "out dx, al",

            "mov edx, 0x01f4",       // port to send bit 8-15 of LBA
            "mov eax, ebx",
            "shr eax, 8",
            "out dx, al",

            "mov edx, 0x01f5",       // port to send bit 16-23 of LBA
            "mov eax, ebx",
            "shr eax, 16",
            "out dx, al",

            "mov edx, 0x01f7",       // command port
            "mov al, 0x20",          // read with retry
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
        );
        asm!("pop edi", "pop edx", "pop ecx", "pop ebx", "pop eax", "popfd")
    }
}

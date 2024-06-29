use core::arch::asm;

pub fn print(message: &str) {
    unsafe {
        asm!(
            "mov si, {0:x}",
            "mov bx, 0",
            "2:",
            "lodsb",
            "cmp al, 0",
            "je 3f",
            "mov ah, 0xe",
            "int 0x10",
            "jmp 2b",
            "3:",
            in(reg) message.as_ptr()
        )
    }
}

pub fn read_sectors(disk: u8, from: u8, sectors: u8, target: u16) -> u8 {
    unsafe {
        let mut error_code: u8;
        asm!(
            "mov ah, 0x2",              // read sector command
            "mov ch, 0x0",              // cylinder
            "mov dh, 0x0",              // head number
            "int 0x13",
            "jc halt",
            in("bx") target,
            in("al") sectors,           // number of sectors to read
            in("cl") from,              // start from nth sector
            in("dl") disk,
            out("ah") error_code,
        );
        error_code
    }
}

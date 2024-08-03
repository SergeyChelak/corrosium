use core::arch::asm;

pub fn cli() {
    unsafe { asm!("cli") }
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}

type Port = u16;

/// read one byte from the given port
pub fn insb(port: Port) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "push eax",
            "xor eax, eax",
            "in al, dx",
            "pop eax",
            in("dx") port,
            out("al") value
        )
    }
    value
}

/// read word from given port 
pub fn insw(port: Port) -> u16 {
    let value: u16;
    unsafe {
        asm!(
            "push eax",
            "xor eax, eax",
            "in al, dx",
            "pop eax",
            in("dx") port,
            out("ax") value
        )
    }
    value
}

pub fn outb(port: Port, value: u8) {
    unsafe {
        asm!(
            "push eax",
            "xor eax, eax", // ???
            "out dx, al",
            "pop eax",
            in("dx") port,
            in("al") value
        )
    }
}

pub fn outw(port: Port, value: u16) {
    unsafe {
        asm!(
            "push eax",
            "xor eax, eax", // ???
            "out dx, ax",
            "pop eax",
            in("dx") port,
            in("ax") value
        )
    }
}
use core::arch::asm;

pub fn cli() {
    unsafe { asm!("cli") }
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}

pub type PortAddress = u16;

/// read one byte from the given port
pub fn inb(port: PortAddress) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value
        )
    }
    value
}

/// read word from given port
pub fn inw(port: PortAddress) -> u16 {
    let value: u16;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("ax") value
        )
    }
    value
}

pub fn outb(port: PortAddress, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value
        )
    }
}

pub fn outw(port: PortAddress, value: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") value
        )
    }
}

pub fn io_wait() {
    outb(0x80, 0)
}

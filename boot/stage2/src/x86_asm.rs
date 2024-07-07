use core::arch::asm;

pub fn out_b(port: u16, byte: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") byte
        )
    }
}

pub fn in_b(port: u16) -> u8 {
    let byte: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") byte
        )
    }
    byte
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}

pub fn cli() {
    unsafe { asm!("cli") }
}

pub fn jump(address: usize) {
    unsafe { asm!("jmp {0:e}", in(reg) address) }
}

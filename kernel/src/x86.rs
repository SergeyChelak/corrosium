use core::arch::asm;

pub fn cli() {
    unsafe { asm!("cli") }
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}

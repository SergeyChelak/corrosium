#![no_std]

pub mod io;
use core::arch::asm;

pub use io::*;

/// halt cpu
pub fn hlt() {
    unsafe { asm!("hlt") }
}

/// disable interruptions
pub fn cli() {
    unsafe { asm!("cli") }
}

/// enable interruptions
pub fn sti() {
    unsafe { core::arch::asm!("sti") }
}

pub fn jump(address: usize) {
    unsafe { asm!("jmp {0:e}", in(reg) address) }
}

pub fn spin_forever() -> ! {
    cli();
    loop {
        hlt()
    }
}

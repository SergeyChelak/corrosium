use core::arch::asm;

pub fn hlt() {
    unsafe { asm!("hlt") }
}

pub fn cli() {
    unsafe { asm!("cli") }
}

pub fn jump(address: *const u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address as u16);
    }
}

pub fn fast_a20() {
    unsafe { asm!("in al, 0x92", "or al, 2", "out 0x92, al",) }
}

// #[repr(C, packed)]
// struct GlobalDescriptorTable {
//     //
// }

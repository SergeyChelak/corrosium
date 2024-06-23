use core::{arch::asm, mem};

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

pub static GDT: GlobalDescriptorTable = {
    let limit: u64 = 0xffff | 0xf << 48;
    let base: u64 = 0;
    // P | DPL | S | E | DC | RW | A
    let access = 0b1 << 47 | 0b00 << 46 | 0b1 << 44 | 0b0 << 43 | 0b0 << 42 | 0b1 << 41 | 0b0 << 40;
    // G | DB | L
    let flags = 0b1 << 55 | 0b1 << 54 | 0b0 << 53;

    let executable = 0b1 << 43;
    GlobalDescriptorTable {
        null: 0,
        code: limit | base | access | executable | flags,
        data: limit | base | access | flags,
    }
};

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    null: u64,
    code: u64,
    data: u64,
}

impl GlobalDescriptorTable {
    pub fn load(&self) {
        let descriptor = GDTDescriptor {
            size: (mem::size_of::<Self>() - 1) as u16,
            ptr: self,
        };
        unsafe { asm!("lgdt [{0:e}]", in(reg) &descriptor) }
    }
}

#[repr(C, packed)]
struct GDTDescriptor {
    size: u16,
    ptr: *const GlobalDescriptorTable,
}

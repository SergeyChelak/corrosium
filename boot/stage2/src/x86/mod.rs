use core::arch::asm;

mod gdt;

pub use gdt::load_flat_mem_gdt;

pub fn fast_a20() {
    unsafe { asm!("in al, 0x92", "or al, 2", "out 0x92, al",) }
}

#[no_mangle]
pub fn halt() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt")
        }
    }
}

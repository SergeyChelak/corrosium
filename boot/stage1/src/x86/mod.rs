use core::arch::asm;

mod gdt;

pub use gdt::load_flat_mem_gdt;

pub fn fast_a20() {
    unsafe { asm!("in al, 0x92", "or al, 2", "out 0x92, al",) }
}

pub fn halt() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt")
        }
    }
}

pub fn jump(address: u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address);
    }
}

pub fn protected_mode() {
    unsafe {
        asm!("mov eax, cr0", "or eax, 0x1", "mov cr0, eax",);
        asm!("ljmp $0x8, $2f", "2:", options(att_syntax));
        asm!(
            ".code32",
            "mov eax, 0x10",
            "mov ds, eax",
            "mov es, eax",
            "mov ss, eax",
            "mov ebp, 0x7c00",
            "mov esp, ebp",
        )
    }
}

#![no_std]
#![no_main]

mod gdt;
mod text_buffer;

use core::arch::asm;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    text_buffer::clear();
    let mut writer = text_buffer::Writer::new();
    writer.write_string("Stage 2\n");
    fast_a20();
    writer.write_string("a20 enabled\n");
    gdt::load_flat_mem_gdt();
    writer.write_string("flat memory gdt loaded\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn fast_a20() {
    unsafe { asm!("in al, 0x92", "or al, 2", "out 0x92, al",) }
}

#![no_std]
#![no_main]

mod text_buffer;
mod x86;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    text_buffer::clear();
    let mut writer = text_buffer::Writer::new();
    writer.write_string("Stage 2\n");
    x86::fast_a20();
    writer.write_string("a20 enabled\n");
    x86::load_flat_mem_gdt();
    writer.write_string("flat memory gdt loaded\n");
    x86::halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    x86::halt()
}

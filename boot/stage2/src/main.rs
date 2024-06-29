#![no_std]
#![no_main]

mod text_buffer;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    text_buffer::clear();
    let mut writer = text_buffer::Writer::new();
    writer.write_string("Stage 2\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

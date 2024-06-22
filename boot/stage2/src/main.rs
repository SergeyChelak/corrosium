#![no_std]
#![no_main]

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    // Bootloader stage #2
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

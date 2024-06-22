#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Bootloader stage #1
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

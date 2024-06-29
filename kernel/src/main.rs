#![no_std]
#![no_main]

#[no_mangle]
#[link_section = ".kernel_start"]
pub extern "C" fn kernel_main() -> ! {
    // green background, white foreground
    let clr = (2 as u8) << 4 | (15 as u8);
    let value = (b'K' as u16) | (clr as u16) << 8;
    unsafe {
        core::ptr::write_volatile(0xb8000 as *mut u16, value);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

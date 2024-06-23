#![no_std]
#![no_main]

mod bios;

use core::arch::global_asm;

global_asm!(include_str!("main.asm"));

#[no_mangle]
pub extern "C" fn _stage1() -> ! {
    bios::print("Bootloader stage #1\r\n\0");

    bios::print("Loading stage 2...\r\n\0");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

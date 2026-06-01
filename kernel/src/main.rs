#![no_std]
#![no_main]

use bootinfo::*;

#[unsafe(no_mangle)]
pub fn _start(_boot_info: &BootInfo) -> ! {
    loop {
        // no op
    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // no op
    }
}

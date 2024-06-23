#![no_std]
#![no_main]

mod bios;
mod x86;

use core::arch::global_asm;

global_asm!(include_str!("main.asm"));

extern "C" {
    static _next_stage: u16;
}

#[no_mangle]
pub extern "C" fn _stage1() -> ! {
    bios::print("Bootloader stage #1\r\n\0");

    let next_stage: *const u16 = unsafe { &_next_stage };
    x86::jump(next_stage);
    halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt()
}

fn halt() -> ! {
    bios::print("[WARN] halted\r\n\0");
    x86::cli();
    loop {
        x86::hlt()
    }
}

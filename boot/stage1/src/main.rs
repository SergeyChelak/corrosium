#![no_std]
#![no_main]

mod bios;

use core::arch::{asm, global_asm};

use bios::print;

const MAX_NEXT_STAGE_SECTORS: u8 = 63;

global_asm!(include_str!("main.asm"));

extern "C" {
    static _next_stage: u16;
}

#[no_mangle]
pub extern "C" fn _stage1() -> ! {
    let disk_id = unsafe {
        let number: u8;
        asm!("mov {0}, dl", out(reg_byte) number);
        number
    };
    let stage2: *const u16 = unsafe { &_next_stage };
    let target = stage2 as u16;
    let error_code = bios::read_sectors(disk_id, 2, MAX_NEXT_STAGE_SECTORS, target);
    if error_code != 0x0 {
        print("! Disk read error\n\r\0");
        halt();
    }
    print("2nd stage loaded\r\n\0");
    jump(target);
    halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt()
}

#[no_mangle]
fn halt() -> ! {
    print("* Halted\r\n\0");
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt")
        }
    }
}

fn jump(address: u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address);
    }
}

#![no_std]
#![no_main]

mod bios;

use core::{
    arch::{asm, global_asm},
    ptr::addr_of,
};

use bios::print;

const MAX_NEXT_STAGE_SECTORS: u8 = 63;

global_asm!(include_str!("main.asm"));

fn next_stage() -> u16 {
    extern "C" {
        #[link_name = "_next_stage"]
        static next_stage: u16;
    }
    unsafe { addr_of!(next_stage) as u16 }
}

#[no_mangle]
pub extern "C" fn _stage1() -> ! {
    let disk_id = unsafe {
        let number: u8;
        asm!("mov {0}, dl", out(reg_byte) number);
        number
    };
    let stage2 = next_stage();
    let error_code = bios::read_sectors(disk_id, 2, MAX_NEXT_STAGE_SECTORS, stage2);
    if error_code != 0x0 {
        print("! Disk read error\n\r\0");
        halt();
    }
    print("2nd stage loaded\r\n\0");
    jump(stage2);
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

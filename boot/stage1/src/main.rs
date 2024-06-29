#![no_std]
#![no_main]

mod bios;
mod x86;

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
    bios::read_sectors(disk_id, 2, MAX_NEXT_STAGE_SECTORS, stage2);
    print("2nd stage loaded\r\n\0");
    x86::fast_a20();
    x86::load_flat_mem_gdt();
    x86::protected_mode();
    x86::jump(stage2);
    halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt()
}

#[no_mangle]
fn halt() -> ! {
    print("* Halted\r\n\0");
    x86::halt();
}

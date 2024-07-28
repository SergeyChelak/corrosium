#![no_std]
#![no_main]

use fat::DirectoryEntry;
use utils::checksum;

mod asm86;
mod ata;
mod debug;
mod fat;
mod text_buffer;
mod utils;

const KERNEL_FILE_NAME: [u8; 11] = [
    b'K', b'E', b'R', b'N', b'E', b'L', b' ', b' ', b'B', b'I', b'N',
];
const KERNEL_TARGET_ADDR: usize = 0x100_000;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    main()
}

fn main() -> ! {
    text_buffer::clear();
    println!("[stage 2] protected mode");
    let result = fat::FAT::new();
    let Ok(fat) = result else {
        handle_error(result.err().unwrap())
    };
    // debug::print_header_info(&fat.header);
    let Some(entry) = kernel_entry(&fat) else {
        panic!("kernel not found");
    };

    debug::print_entry(&entry);
    fat.load_entry(&entry, KERNEL_TARGET_ADDR);
    // debug::dump_memory(KERNEL_TARGET_ADDR, 20);
    let checksum = checksum(KERNEL_TARGET_ADDR, entry.file_size as usize);
    println!("Checksum {}", checksum);
    /*
    // asm86::jump(KERNEL_TARGET_ADDR);
     */
    halt()
}

fn handle_error(error: fat::FatError) -> ! {
    use fat::FatError::*;
    match error {
        BadFatSize(size) => println!("FAT size {size} is not in valid range [1..20]"),
        BadSectorSize(size) => println!("invalid sector size: {size} bytes"),
    }
    halt()
}

fn kernel_entry(fat: &fat::FAT) -> Option<DirectoryEntry> {
    let predicate = |entry: &DirectoryEntry| {
        entry
            .name
            .iter()
            .zip(KERNEL_FILE_NAME.iter())
            .all(|(a, b)| *a == *b)
    };
    fat.find_root_entry(predicate)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    halt()
}

fn halt() -> ! {
    println!("[Halted]");
    asm86::cli();
    loop {
        asm86::hlt()
    }
}

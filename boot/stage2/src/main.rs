#![no_std]
#![no_main]

use debug::dump_memory;
use fat::{find_root_entry, DirectoryEntry, FatHeader, SECTOR_SIZE};

mod ata;
mod debug;
mod fat;
mod text_buffer;
mod x86_asm;

const FAT_TABLE_MAX_SECTORS: usize = 20;
const KERNEL_FILE_NAME: [u8; 11] = [
    b'K', b'E', b'R', b'N', b'E', b'L', b' ', b' ', b'B', b'I', b'N',
];
const KERNEL_TARGET_ADDR: u32 = 0x100_000;

extern "C" {
    #[link_name = "_fat_table"]
    static fat_table: u32;
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    text_buffer::clear();
    println!("[Stage 2] Protected mode");
    let header = fat::load_header();
    if header.bytes_per_sector as usize != fat::SECTOR_SIZE {
        let size = header.bytes_per_sector;
        panic!("Invalid sector size: {size} bytes");
    }
    if header.sectors_per_fat as usize > FAT_TABLE_MAX_SECTORS || header.sectors_per_fat == 0 {
        let size = header.sectors_per_fat;
        panic!("FAT size is invalid: {size}. Expected 1..20");
    }
    // debug::print_header_info(&header);

    let Some(entry) = kernel_entry(&header) else {
        panic!("Kernel not found");
    };
    debug::print_entry(&entry);

    // load fat
    let fat_table_addr: *const u32 = unsafe { &fat_table };
    ata::load(
        header.reserved_sectors_count as u32,
        header.sectors_per_fat as u8,
        fat_table_addr,
    );
    println!("FAT loaded");

    // load kernel
    let lba_data_region = header.data_region_start_sector();
    let mut current_cluster = entry.get_start_cluster();

    let fat = |i: u32| -> u8 {
        unsafe {
            let addr = fat_table_addr as u32 + i;
            core::ptr::read(addr as *const _)
        }
    };
    let mut addr = KERNEL_TARGET_ADDR;
    loop {
        // first two clusters are reserved
        let lba = lba_data_region + (current_cluster - 2) * header.sectors_per_cluster as u16;
        ata::load(lba as u32, header.sectors_per_cluster, addr as *const _);
        addr += header.sectors_per_cluster as u32 * SECTOR_SIZE as u32;
        let byte_idx = 2 * current_cluster as u32;
        if fat(byte_idx + 1) == 0xff && fat(byte_idx) == 0xff {
            break;
        }
        current_cluster += 1;
    }

    println!("Kernel loaded");
    dump_memory(KERNEL_TARGET_ADDR, 20);
    x86_asm::jump(KERNEL_TARGET_ADDR);
    halt()
}

fn kernel_entry(header: &FatHeader) -> Option<DirectoryEntry> {
    let predicate = |entry: &DirectoryEntry| {
        entry
            .name
            .iter()
            .zip(KERNEL_FILE_NAME.iter())
            .all(|(a, b)| *a == *b)
    };
    find_root_entry(header, predicate)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    halt()
}

fn halt() -> ! {
    println!("[Halted]");
    x86_asm::cli();
    loop {
        x86_asm::hlt()
    }
}

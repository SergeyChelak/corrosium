#![no_std]
#![no_main]

use core::arch::asm;

mod ata;
mod fat16;
mod text_buffer;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _stage2() -> ! {
    text_buffer::clear();
    println!("Stage 2: Protected mode");
    let fat_header = fat16::load_header();
    print_header_info(&fat_header);
    fat16::read_root_directory(&fat_header);
    halt()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    halt()
}

fn halt() -> ! {
    println!("* Halted");
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt")
        }
    }
}

fn print_header_info(header: &fat16::FatHeader) {
    println!("\n* FAT Header *");
    {
        print!("OEM: ");
        header
            .oem_name
            .iter()
            .map(|x| *x as char)
            .for_each(|x| print!("{x}"));
        println!();
    }
    {
        let val = header.bytes_per_sector;
        println!("Bytes per sector: {}", val);
    }
    {
        let val = header.sectors_per_cluster;
        println!("Sector per cluster: {}", val);
    }
    {
        let val = header.reserved_sectors_count;
        println!("Reserved sectors: {}", val);
    }
    {
        let val = header.fat_count;
        println!("FATs count: {}", val);
    }
    {
        let val = header.root_directory_entries;
        println!("Root directory entries: {}", val);
    }
    {
        let val = header.total_sectors;
        println!("Total sectors low: {:x}h", val);
    }
    {
        let val = header.media_descriptor_type;
        println!("Media descriptor type: {:x}h", val);
    }
    {
        let val = header.sectors_per_fat;
        println!("Sectors per FAT: {:}", val);
    }
    {
        let val = header.sectors_per_track;
        println!("Sectors per track: {:}", val);
    }
    {
        let val = header.number_of_heads;
        println!("Heads number: {:}", val);
    }
    {
        let val = header.hidden_sectors;
        println!("Hidden sectors: {:}", val);
    }
    {
        let val = header.total_sectors_32;
        println!("Total sectors: {:x}h", val);
    }
}

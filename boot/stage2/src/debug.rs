use vga_buffer::*;

use crate::fat::*;

#[allow(dead_code)]
pub fn print_header_info(header: &FatHeader) {
    {
        print!("OEM: ");
        println_str_buffer(&header.oem_name);
    }
    println!("Bytes per sector: {}", header.bytes_per_sector as u16);
    println!("Sectors per cluster: {}", header.sectors_per_cluster as u8);
    println!("Reserved sectors: {}", header.reserved_sectors_count as u16);
    println!("FATs count: {}", header.fat_count as u8);
    println!(
        "Root directory entries: {}",
        header.root_directory_entries as u16
    );
    println!("Total sectors low: {:x}h", header.total_sectors as u16);
    println!(
        "Media descriptor type: {:x}h",
        header.media_descriptor_type as u8
    );
    println!("Sectors per FAT: {:}", header.sectors_per_fat as u16);
    println!("Sectors per track: {:}", header.sectors_per_track as u16);
    println!("Heads number: {:}", header.number_of_heads as u16);
    println!("Hidden sectors: {:}", header.hidden_sectors as u32);
    println!("Total sectors: {:x}h", header.total_sectors_32 as u32);
}

#[allow(dead_code)]
pub fn print_entry(entry: &crate::fat::DirectoryEntry) {
    let attr = entry.attributes;
    let size = entry.file_size;
    print!("'");
    print_str_buffer(&entry.name);
    let cluster = entry.get_start_cluster();
    println!("' size: {} | attr: 0x{attr:x} | cluster: {}", size, cluster);
}

pub fn print_str_buffer(buffer: &[u8]) {
    buffer
        .iter()
        .map(|x| *x as char)
        .for_each(|x| print!("{x}"));
}

pub fn println_str_buffer(buffer: &[u8]) {
    print_str_buffer(buffer);
    println!();
}

#[allow(dead_code)]
pub fn dump_memory(address: usize, count: usize) {
    for i in 0..count {
        let addr = address + i;
        let byte: u8 = unsafe { core::ptr::read(addr as *const _) };
        print!("{:<4x}", byte);
    }
    println!();
}

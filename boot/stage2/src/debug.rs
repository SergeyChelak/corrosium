use crate::{fat::*, print, println};

pub fn print_header_info(header: &FatHeader) {
    {
        print!("OEM: ");
        println_str_buffer(&header.oem_name);
    }
    {
        let val = header.bytes_per_sector;
        println!("Bytes per sector: {}", val);
    }
    {
        let val = header.sectors_per_cluster;
        println!("Sectors per cluster: {}", val);
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

pub fn print_entry(entry: &crate::fat::DirectoryEntry) {
    let attr = entry.attributes;
    let size = entry.file_size;
    print!("'");
    print_str_buffer(&entry.name);
    println!("' size: {}, attributes: {attr:x}", size);
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

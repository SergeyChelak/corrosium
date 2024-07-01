use crate::{ata, print, println};
use core::{mem, ptr::addr_of};

/// Common FAT-family boot Sector and BIOS parameter blocks
#[repr(C, packed)]
pub struct FatHeader {
    pub jump_instruction: [u8; 3],
    pub oem_name: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors_count: u16,
    pub fat_count: u8,
    pub root_directory_entries: u16,
    pub total_sectors: u16,
    pub media_descriptor_type: u8,
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16, // for interrupt 0x13
    pub number_of_heads: u16,   // for interrupt 0x13
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,
}

impl FatHeader {
    pub fn root_directory_start_sector(&self) -> u16 {
        self.reserved_sectors_count + self.sectors_per_fat * self.fat_count as u16
    }

    pub fn root_directory_size_bytes(&self) -> usize {
        self.root_directory_entries as usize * mem::size_of::<DirectoryEntry>()
    }

    pub fn root_directory_size_sectors(&self) -> usize {
        self.sectors_for_size(self.root_directory_size_bytes())
    }

    pub fn sectors_for_size(&self, size: usize) -> usize {
        let bps = self.bytes_per_sector as usize;
        let mut sectors = size / bps;
        if size % bps != 0 {
            sectors += 1;
        }
        sectors
    }
}

#[repr(C, packed)]
pub struct DirectoryEntry {
    pub name: [u8; 11],
    attributes: u8,
    reserved: u8,
    creation_time_tenth: u8,
    creation_time: u16,
    creation_date: u16,
    last_access_date: u16,
    first_cluster_high: u16,
    write_time: u16,
    write_date: u16,
    first_cluster_low: u16,
    pub file_size: u32,
}

pub fn load_header() -> FatHeader {
    let buffer = [0u8; 512];
    ata::load(0, 1, addr_of!(buffer) as u32);
    let header: FatHeader = unsafe { core::ptr::read(buffer.as_ptr() as *const _) };
    header
}

pub fn read_root_directory(header: &FatHeader) {
    println!("\n* Root directory *");
    println!(
        "Root start sector: {}",
        header.root_directory_start_sector()
    );
    let entries = header.root_directory_entries as usize;
    println!(
        "Root directory sectors size: {}",
        header.root_directory_size_sectors(),
    );

    let entry_size = mem::size_of::<DirectoryEntry>();
    if header.bytes_per_sector as usize % entry_size != 0 {
        println!("Sector size isn't expected");
        return;
    }

    let mut lba = header.root_directory_start_sector() as u32;
    let mut items = 0;
    let count = header.bytes_per_sector as usize / entry_size;
    println!("count = {count}");
    while items < entries {
        let buffer = [0u8; 1024];
        ata::load(lba, 1, addr_of!(buffer) as u32);
        for i in 0..count {
            let slice = &buffer[entry_size * i..entry_size * (i + 1)];
            let entry: DirectoryEntry = unsafe { core::ptr::read(slice.as_ptr() as *const _) };
            if entry.name[0] == 0 {
                continue;
            }
            // item is unused
            if entry.name[0] == 0xe5 {
                continue;
            }
            {
                let size = entry.file_size;
                print!("Entry: '");
                entry
                    .name
                    .iter()
                    .map(|x| *x as char)
                    .for_each(|x| print!("{x}"));
                println!("' size: {} @ {lba}", size);
            }
        }
        lba += 1;
        items += count;
    }
    println!("done");
}

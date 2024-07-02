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

const ATTR_READ_ONLY: u8 = 0x01;
const ATTR_HIDDEN: u8 = 0x02;
const ATTR_SYSTEM: u8 = 0x04;
const ATTR_VOLUME_ID: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
const ATTR_ARCHIVE: u8 = 0x20;
const ATTR_LONG_NAME: u8 = ATTR_READ_ONLY | ATTR_HIDDEN | ATTR_SYSTEM | ATTR_VOLUME_ID;

#[repr(C, packed)]
pub struct DirectoryEntry {
    name: [u8; 11],
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
    file_size: u32,
}

impl DirectoryEntry {
    fn is_long_name(&self) -> bool {
        self.attributes == ATTR_LONG_NAME
    }

    fn is_empty(&self) -> bool {
        self.name[0] == 0 || self.name[0] == 0xe5
    }

    fn is_directory(&self) -> bool {
        self.attributes & ATTR_DIRECTORY == 1
    }
}

pub fn load_header() -> FatHeader {
    let buffer = [0u8; 512];
    let addr = addr_of!(buffer) as *mut u8;
    ata::load(0, 1, addr);
    let header: FatHeader = unsafe { core::ptr::read_volatile(addr as *const _) };
    header
}

pub fn read_root_directory(header: &FatHeader) {
    println!("\n* Root directory *");
    let entry_size = mem::size_of::<DirectoryEntry>();
    if header.bytes_per_sector as usize % entry_size != 0 {
        println!("Sector size isn't expected");
        return;
    }
    let count = header.bytes_per_sector as usize / entry_size;
    let mut lba = header.root_directory_start_sector() as u32;
    let mut items = 0;
    let entries = header.root_directory_entries as usize;
    let buffer = [0u8; 512];
    while items < entries {
        ata::load(lba, 1, addr_of!(buffer) as *const _);
        for i in 0..count {
            let slice = &buffer[entry_size * i..];
            let entry: DirectoryEntry =
                unsafe { core::ptr::read_volatile(slice.as_ptr() as *const _) };
            if entry.is_empty() || entry.is_long_name() || entry.is_directory() {
                continue;
            }
            {
                let attr = entry.attributes;
                let size = entry.file_size;
                print!("Entry: '");
                entry
                    .name
                    .iter()
                    .map(|x| *x as char)
                    .for_each(|x| print!("{x}"));
                println!("' size: {}, attributes: {attr:x}", size);
            }
        }
        lba += 1;
        items += count;
    }
}

mod header;

use core::mem;

pub use header::*;

mod entry;
pub use entry::*;

use crate::ata::{self};

pub const SECTOR_SIZE: usize = 0x200;

pub fn load_header() -> FatHeader {
    let addr = ata::load_into_buffer(0, 1);
    let header: FatHeader = unsafe { core::ptr::read_volatile(addr as *const _) };
    header
}

pub fn find_root_entry(
    header: &FatHeader,
    predicate: impl Fn(&DirectoryEntry) -> bool,
) -> Option<DirectoryEntry> {
    let entry_size = mem::size_of::<FatHeader>();
    let entries_per_sector = header.bytes_per_sector as usize / entry_size;
    let mut lba = header.root_directory_start_sector() as u32;
    let mut items = 0;
    let entries = header.root_directory_entries as usize;
    while items < entries {
        let buffer_addr = ata::load_into_buffer(lba, 1);
        for i in 0..entries_per_sector {
            let addr = buffer_addr as u32 + (entry_size * i) as u32;
            let entry: DirectoryEntry = unsafe { core::ptr::read_volatile(addr as *const _) };
            if entry.is_empty() || entry.is_long_name() || entry.is_directory() {
                continue;
            }
            if predicate(&entry) {
                return Some(entry);
            }
        }
        lba += 1;
        items += entries_per_sector;
    }
    None
}

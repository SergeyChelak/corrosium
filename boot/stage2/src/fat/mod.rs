mod entry;
mod header;

use core::hint::black_box;
use core::mem;
use core::ptr::*;

pub use entry::*;
pub use header::*;

use crate::ata;

const SECTOR_SIZE: usize = 0x200;
const FAT_TABLE_MAX_SECTORS: usize = 20;

extern "C" {
    #[link_name = "_fat_table"]
    static fat_table: usize;
}

pub enum FatError {
    BadSectorSize(usize),
    BadFatSize(usize),
}

pub struct FAT {
    pub header: FatHeader,
    pub table_address: *const usize,
}

impl FAT {
    pub fn new() -> Result<FAT, FatError> {
        let addr = ata::load_into_buffer(0, 1);
        let header: FatHeader = unsafe { read_volatile(addr as *const _) };
        {
            let sector_size = header.bytes_per_sector as usize;
            if sector_size != SECTOR_SIZE {
                return Err(FatError::BadSectorSize(sector_size));
            }
            let fat_size = header.sectors_per_fat as usize;
            if fat_size > FAT_TABLE_MAX_SECTORS || fat_size == 0 {
                return Err(FatError::BadFatSize(fat_size));
            }
        }
        // load fat
        let table_address: *const usize = unsafe { &fat_table };
        ata::load(
            header.reserved_sectors_count as usize,
            header.sectors_per_fat as u8,
            table_address,
        );
        Ok(Self {
            header,
            table_address,
        })
    }

    pub fn find_root_entry(
        &self,
        predicate: impl Fn(&DirectoryEntry) -> bool,
    ) -> Option<DirectoryEntry> {
        let entry_size = mem::size_of::<FatHeader>();
        let entries_per_sector = self.header.bytes_per_sector as usize / entry_size;
        let mut lba = self.header.root_directory_start_sector() as usize;
        let mut items = 0;
        let entries = self.header.root_directory_entries as usize;
        while items < entries {
            let buffer_addr = ata::load_into_buffer(lba, 1);
            for i in 0..entries_per_sector {
                let addr = buffer_addr as usize + entry_size * i;
                let entry: DirectoryEntry = unsafe { read_volatile(addr as *const _) };
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

    pub fn load_entry(&self, entry: &DirectoryEntry, target: usize) {
        let sector_size = self.header.sectors_per_cluster as usize;
        let lba_data_region = self.header.data_region_start_sector() as usize;
        let mut current_cluster = entry.get_start_cluster() as usize;
        let mut addr = target;
        loop {
            // first two clusters are reserved
            let lba = lba_data_region + (current_cluster - 2) * sector_size;
            ata::load(lba, self.header.sectors_per_cluster, addr as *const _);
            if self.is_terminal_cluster(current_cluster) {
                break;
            }
            addr += sector_size * SECTOR_SIZE;
            current_cluster += 1;
        }
    }

    fn is_terminal_cluster(&self, cluster: usize) -> bool {
        let position = self.table_address as usize + 2 * cluster;
        let marker = unsafe {
            black_box(
                (read(position as *const u8) as u16) << 8
                    | read((position + 1) as *const u8) as u16,
            )
        };
        marker == 0xffff
    }
}

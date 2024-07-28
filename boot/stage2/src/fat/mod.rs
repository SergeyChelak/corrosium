pub mod entry;
pub mod header;

use core::mem;
use core::ptr::*;

pub use entry::*;
pub use header::*;

use crate::ata;

const SECTOR_SIZE: usize = 0x200;
const FAT_TABLE_MAX_SECTORS: usize = 20;
const FAT_TABLE_SIZE: usize = FAT_TABLE_MAX_SECTORS * SECTOR_SIZE;

type FatTable = [u8; FAT_TABLE_SIZE];

pub enum FatError {
    BadSectorSize(usize),
    BadFatSize(usize),
}

pub struct FAT {
    pub header: FatHeader,
    pub table: FatTable,
}

impl FAT {
    pub fn new() -> Result<FAT, FatError> {
        let header = load_header();
        if let Err(error) = validate_header(&header) {
            return Err(error);
        }
        let table = load_fat(&header);
        Ok(Self { header, table })
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
        let buffer = [0u8; SECTOR_SIZE];
        let buffer_addr = addr_of!(buffer) as *mut _;
        while items < entries {
            ata::load(lba, 1, buffer_addr);
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
        let cluster_size = self.header.sectors_per_cluster as usize;
        let lba_data_region = self.header.data_region_start_sector() as usize;
        let mut cluster = entry.get_start_cluster() as usize;
        let mut addr = target;
        let mut is_terminal_cluster = false;
        while !is_terminal_cluster {
            let index = cluster * 2;
            is_terminal_cluster = self.table[index] == 0xff && self.table[index + 1] == 0xff;
            // first two clusters are reserved
            let lba = lba_data_region + (cluster - 2) * cluster_size;
            ata::load(lba, cluster_size as u8, addr as *mut _);
            addr += cluster_size * SECTOR_SIZE;
            cluster += 1;
        }
    }
}

fn load_header() -> FatHeader {
    let buffer = [0u8; SECTOR_SIZE];
    let buffer_addr = addr_of!(buffer) as *mut usize;
    ata::load(0, 1, buffer_addr);
    unsafe { core::ptr::read_volatile(buffer_addr as *const _) }
}

fn validate_header(header: &FatHeader) -> Result<(), FatError> {
    let sector_size = header.bytes_per_sector as usize;
    if sector_size != SECTOR_SIZE {
        return Err(FatError::BadSectorSize(sector_size));
    }
    let fat_size = header.sectors_per_fat as usize;
    if fat_size > FAT_TABLE_MAX_SECTORS || fat_size == 0 {
        return Err(FatError::BadFatSize(fat_size));
    }
    return Ok(());
}

fn load_fat(header: &FatHeader) -> FatTable {
    let fat = [0u8; FAT_TABLE_SIZE];
    let fat_addr = addr_of!(fat) as *mut usize;
    ata::load(
        header.reserved_sectors_count as usize,
        header.sectors_per_fat as u8,
        fat_addr,
    );
    fat
}

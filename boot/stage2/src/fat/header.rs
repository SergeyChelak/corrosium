use core::mem;

use super::DirectoryEntry;

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
    pub fn sectors_for_size(&self, size: usize) -> usize {
        let bps = self.bytes_per_sector as usize;
        let mut sectors = size / bps;
        if size % bps != 0 {
            sectors += 1;
        }
        sectors
    }

    pub fn root_directory_start_sector(&self) -> u16 {
        self.reserved_sectors_count + self.sectors_per_fat * self.fat_count as u16
    }

    pub fn data_region_start_sector(&self) -> u16 {
        self.root_directory_start_sector() + self.root_directory_size_sectors() as u16
    }

    pub fn root_directory_size_bytes(&self) -> usize {
        self.root_directory_entries as usize * mem::size_of::<DirectoryEntry>()
    }

    pub fn root_directory_size_sectors(&self) -> usize {
        self.sectors_for_size(self.root_directory_size_bytes())
    }
}

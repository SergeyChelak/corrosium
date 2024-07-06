const ATTR_READ_ONLY: u8 = 0x01;
const ATTR_HIDDEN: u8 = 0x02;
const ATTR_SYSTEM: u8 = 0x04;
const ATTR_VOLUME_ID: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
// const ATTR_ARCHIVE: u8 = 0x20;
const ATTR_LONG_NAME: u8 = ATTR_READ_ONLY | ATTR_HIDDEN | ATTR_SYSTEM | ATTR_VOLUME_ID;

#[repr(C, packed)]
pub struct DirectoryEntry {
    pub name: [u8; 11],
    pub attributes: u8,
    pub reserved: u8,
    pub creation_time_tenth: u8,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    pub first_cluster_high: u16,
    pub write_time: u16,
    pub write_date: u16,
    pub first_cluster_low: u16,
    pub file_size: u32,
}

impl DirectoryEntry {
    pub fn is_long_name(&self) -> bool {
        self.attributes == ATTR_LONG_NAME
    }

    pub fn is_empty(&self) -> bool {
        self.name[0] == 0 || self.name[0] == 0xe5
    }

    pub fn is_directory(&self) -> bool {
        self.attributes & ATTR_DIRECTORY == 1
    }

    pub fn get_start_cluster(&self) -> u16 {
        self.first_cluster_high | self.first_cluster_low
    }
}

#![no_std]
mod config_table;
pub use config_table::ConfigTable;

mod framebuffer;
pub use framebuffer::*;

mod memory_map;
pub use memory_map::*;

#[repr(C)]
pub struct BootInfo {
    pub acpi: ConfigTable,
    pub smbios: ConfigTable,
    pub framebuffer: FrameBuffer,
    pub memory_map: MemoryMapInfo,
}

#![no_std]
mod config_table;
pub use config_table::ConfigTable;

mod framebuffer;
pub use framebuffer::*;

mod memory_map;
pub use memory_map::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KernelRange {
    pub start_addr: u64,
    pub end_addr: u64,
}

#[repr(C)]
pub struct BootInfo {
    pub acpi: ConfigTable,
    pub smbios: ConfigTable,
    pub framebuffer: FrameBufferInfo,
    pub memory_map: MemoryMapInfo,
    pub kernel_range: KernelRange,
}

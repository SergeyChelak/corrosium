#![no_std]
// mod config_table;
// pub use config_table::ConfigTable;

// mod framebuffer;
// pub use framebuffer::*;

// mod memory_map;
// pub use memory_map::*;

use platform::address::VirtualAddress;
pub use platform::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KernelLayout {
    pub start_addr: VirtualAddress,
    pub end_addr: VirtualAddress,
}

#[repr(C)]
pub struct BootInfo {
    // pub acpi: ConfigTable,
    // pub smbios: ConfigTable,
    // pub framebuffer: FrameBufferInfo,
    // pub memory_map: MemoryMapInfo,
    // pub kernel_range: KernelRange,
    // pub higher_half_direct_mapping: VirtualAddr,
}

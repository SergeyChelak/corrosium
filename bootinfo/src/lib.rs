#![no_std]
mod config_table;
pub use config_table::ConfigTable;

mod framebuffer;
pub use framebuffer::*;

mod memory_map;
pub use memory_map::*;

#[repr(C)]
#[derive(Default)]
pub struct BootInfo {
    pub acpi: Option<ConfigTable>,
    pub smbios: Option<ConfigTable>,
    pub frame_buffer: Option<FrameBuffer>,
}

impl BootInfo {
    pub fn set_acpi(&mut self, acpi: ConfigTable) {
        if acpi.is_newer_version(&self.acpi) {
            self.acpi = Some(acpi);
        }
    }

    pub fn set_smbios(&mut self, smbios: ConfigTable) {
        if smbios.is_newer_version(&self.smbios) {
            self.smbios = Some(smbios);
        }
    }
}

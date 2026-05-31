#![no_std]
use core::ffi::c_void;

#[repr(C)]
#[derive(Default)]
pub struct BootInfo {
    pub acpi: Option<ConfigTable>,
    pub smbios: Option<ConfigTable>,
    pub frame_buffer: Option<FrameBuffer>,
}

#[repr(C)]
pub struct ConfigTable {
    pub address: *const c_void,
    pub version: u8,
}

impl ConfigTable {
    fn is_newer_version(&self, other: &Option<ConfigTable>) -> bool {
        let Some(other) = other else {
            return true;
        };
        self.version > other.version
    }
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

#[repr(C)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
}

// #[repr(C)]
// pub enum PixelFormat {
//     RGB,
//     BGR,
// }

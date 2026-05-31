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

/// Represents the graphics framebuffer memory where pixels are drawn to the screen.
#[repr(C)]
pub struct FrameBuffer {
    /// The visible width of the screen in pixels.
    pub width: usize,
    /// The visible height of the screen in pixels.
    pub height: usize,
    /// The actual number of pixels in a horizontal row in memory.
    /// This may be larger than `width` due to hardware memory alignment and padding.
    /// When calculating the index of a pixel at (x, y), always use `y * stride + x`.
    pub stride: usize,
    /// The physical base memory address of the framebuffer.
    pub base_address: *mut c_void,
    /// The total size of the framebuffer memory in bytes.
    /// Because of padding, this is typically `stride * height * bytes_per_pixel`.
    pub size: usize,
    /// The color layout format of the pixels in the framebuffer.
    pub pixel_format: PixelFormat,
}

#[repr(C)]
pub enum PixelFormat {
    RGB,
    BGR,
}

use core::ffi::c_void;

#[repr(C)]
pub struct ConfigTable {
    pub address: *const c_void,
    pub version: u8,
}

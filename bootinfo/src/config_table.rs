use core::ffi::c_void;

#[repr(C)]
pub struct ConfigTable {
    pub address: *const c_void,
    pub version: u8,
}

impl ConfigTable {
    pub fn is_newer_version(&self, other: &Option<ConfigTable>) -> bool {
        let Some(other) = other else {
            return true;
        };
        self.version > other.version
    }
}

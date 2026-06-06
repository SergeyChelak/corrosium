use core::{
    marker::PhantomData,
    ops::{Add, Sub},
};

pub type PhysicalAddress = Address<Physical>;
pub type VirtualAddress = Address<Virtual>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Physical;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Virtual;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Address<Kind> {
    addr: u64,
    _marker: PhantomData<Kind>,
}

impl<Kind> Address<Kind> {
    pub const fn new(addr: u64) -> Self {
        Self {
            addr,
            _marker: PhantomData,
        }
    }

    // pub fn from_ptr(ptr: *const core::ffi::c_void) -> Self {
    //     Self::new(ptr as u64)
    // }

    pub const fn as_u64(self) -> u64 {
        self.addr
    }
}

impl From<u64> for Address<Virtual> {
    fn from(addr: u64) -> Self {
        Self::new(addr)
    }
}

impl<Kind> From<*const core::ffi::c_void> for Address<Kind> {
    fn from(addr: *const core::ffi::c_void) -> Self {
        Self::new(addr as u64)
    }
}

impl<Kind> From<*mut core::ffi::c_void> for Address<Kind> {
    fn from(addr: *mut core::ffi::c_void) -> Self {
        Self::new(addr as u64)
    }
}

impl<Kind> Add<u64> for Address<Kind> {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.addr + rhs)
    }
}

impl<Kind> Sub<u64> for Address<Kind> {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        Self::new(self.addr - rhs)
    }
}

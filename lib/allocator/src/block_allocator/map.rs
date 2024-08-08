use core::ptr::{read_volatile, write_volatile};

use super::AllocationMap;

pub struct AreaAllocationMap {
    pub pointer: *mut u8,
    pub size: usize,
}

impl AreaAllocationMap {
    pub fn new(pointer: *mut u8, size: usize) -> Self {
        Self { pointer, size }
    }
}

impl AllocationMap for AreaAllocationMap {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, position: usize) -> Option<u8> {
        if position >= self.size {
            return None;
        }
        let value = unsafe {
            let ptr = self.pointer.offset(position as isize);
            read_volatile(ptr)
        };
        Some(value)
    }

    fn set(&self, position: usize, value: u8) -> bool {
        if position >= self.size {
            return false;
        }
        unsafe {
            let ptr = self.pointer.offset(position as isize);
            write_volatile(ptr, value);
        }
        true
    }
}

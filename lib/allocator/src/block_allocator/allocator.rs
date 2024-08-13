use core::{alloc::GlobalAlloc, ptr::null_mut};

use super::{table::BlockAllocationTable, AllocationMap};

pub struct BlockAllocator<T: AllocationMap> {
    pub table: BlockAllocationTable<T>,
    pub block_size: usize,
    pub pointer: *mut u8,
}

impl<T: AllocationMap> BlockAllocator<T> {
    pub fn new(table: BlockAllocationTable<T>, pointer: *mut u8, block_size: usize) -> Self {
        Self {
            table,
            pointer,
            block_size,
        }
    }

    fn size_to_blocks(&self, bytes: usize) -> usize {
        let mut count = bytes / self.block_size;
        if bytes % self.block_size > 0 {
            count += 1;
        }
        count
    }

    fn address_to_position(&self, ptr: *const u8) -> Option<usize> {
        let base = self.pointer as usize;
        let address = ptr as usize;
        if !(base..base + self.block_size * self.blocks_count()).contains(&address) {
            return None;
        }
        Some((address - base) / self.block_size)
    }

    fn position_to_address(&self, position: usize) -> *mut u8 {
        let offset = position * self.block_size;
        unsafe { self.pointer.offset(offset as isize) }
    }

    fn blocks_count(&self) -> usize {
        self.table.size()
    }
}

unsafe impl<T: AllocationMap> GlobalAlloc for BlockAllocator<T> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        // at this point we ignore aligment
        let blocks = self.size_to_blocks(size);
        let Some(position) = self.table.allocate(blocks) else {
            return null_mut();
        };
        self.position_to_address(position)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let Some(position) = self.address_to_position(ptr) else {
            return;
        };
        _ = self.table.deallocate(position);
    }
}

unsafe impl<T: AllocationMap> Sync for BlockAllocator<T> {}

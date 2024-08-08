mod allocator;
mod map;
mod table;

pub use allocator::BlockAllocator;

pub const TABLE_ENTRY_DEFAULT_VALUE: u8 = 0;

pub trait AllocationMap {
    fn size(&self) -> usize;

    fn get(&self, position: usize) -> Option<u8>;

    fn set(&self, position: usize, value: u8) -> bool;
}

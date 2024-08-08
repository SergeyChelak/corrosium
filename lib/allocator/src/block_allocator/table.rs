use super::{AllocationMap, TABLE_ENTRY_DEFAULT_VALUE};

const TABLE_ENTRY_FLAG_IS_OCCUPIED: u8 = 1 << 0;
const TABLE_ENTRY_FLAG_IS_FIRST: u8 = 1 << 1;
const TABLE_ENTRY_FLAG_HAS_NEXT: u8 = 1 << 2;

pub struct BlockAllocationTable<T: AllocationMap> {
    alloc_map: T,
}

impl<T: AllocationMap> BlockAllocationTable<T> {
    pub fn with_map(alloc_map: T) -> Self {
        Self { alloc_map }
    }

    fn entries(&self) -> usize {
        self.alloc_map.size()
    }

    fn find_position(&self, count: usize) -> Option<usize> {
        let mut block: Option<usize> = None;
        let mut len = 0usize;
        let size = self.entries();
        for index in 0..size {
            let value = self.alloc_map.get(index)?;
            if !is_free(value) {
                block = None;
                len = 0;
                continue;
            }
            if block.is_none() {
                block = Some(index);
            }
            len += 1;
            if len == count {
                break;
            }
        }
        if len < count {
            return None;
        }
        block
    }

    fn mark_allocated(&mut self, position: usize, count: usize) -> bool {
        if count == 0 {
            return false;
        }
        let size = self.entries();
        if position + count > size {
            return false;
        }
        let last = count - 1;
        for i in 0..count {
            let is_first = i == 0;
            let has_next = i < last;
            let mut value = TABLE_ENTRY_DEFAULT_VALUE | TABLE_ENTRY_FLAG_IS_OCCUPIED;
            if is_first {
                value |= TABLE_ENTRY_FLAG_IS_FIRST;
            }
            if has_next {
                value |= TABLE_ENTRY_FLAG_HAS_NEXT;
            }
            self.alloc_map.set(position + i, value);
        }
        true
    }

    pub fn allocate(&mut self, count: usize) -> Option<usize> {
        let position = self.find_position(count)?;
        if self.mark_allocated(position, count) {
            return Some(position);
        }
        None
    }

    pub fn deallocate(&mut self, position: usize) -> bool {
        let Some(value) = self.alloc_map.get(position) else {
            return false;
        };
        // prevent an attempt to release block from arbitrary position
        if !is_first(value) {
            return false;
        }
        let mut idx = position;
        loop {
            let Some(value) = self.alloc_map.get(idx) else {
                break;
            };
            self.alloc_map.set(idx, TABLE_ENTRY_DEFAULT_VALUE);
            if !has_next(value) {
                break;
            }
            idx += 1;
        }
        true
    }
}

fn is_free(value: u8) -> bool {
    value & TABLE_ENTRY_FLAG_IS_OCCUPIED == 0
}

fn has_next(value: u8) -> bool {
    value & TABLE_ENTRY_FLAG_HAS_NEXT != 0
}

fn is_first(value: u8) -> bool {
    value & TABLE_ENTRY_FLAG_IS_FIRST != 0
}

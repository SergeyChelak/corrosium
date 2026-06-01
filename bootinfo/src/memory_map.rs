#[repr(C)]
pub struct MemoryMapInfo {
    pub entries: *mut MemoryMapEntry,
    pub count: usize,
}

pub type MemoryType = u32;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;
pub type MemoryAttribute = u64;

#[repr(C)]
pub struct MemoryMapEntry {
    /// Type of memory occupying this range.
    pub ty: MemoryType,
    // Implicit 32-bit padding.
    /// Starting physical address.
    pub phys_start: PhysicalAddress,
    /// Starting virtual address.
    pub virt_start: VirtualAddress,
    /// Number of 4 KiB pages contained in this range.
    pub page_count: u64,
    /// The capability attributes of this memory range.
    pub att: MemoryAttribute,
}

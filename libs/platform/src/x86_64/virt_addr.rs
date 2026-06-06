use crate::address::VirtualAddress;

impl VirtualAddress {
    pub const fn pml4_index(self) -> usize {
        ((self.as_u64() >> 39) & 0x1FF) as usize
    }

    pub const fn pdpt_index(self) -> usize {
        ((self.as_u64() >> 30) & 0x1FF) as usize
    }

    pub const fn pd_index(self) -> usize {
        ((self.as_u64() >> 21) & 0x1FF) as usize
    }

    pub const fn pt_index(self) -> usize {
        ((self.as_u64() >> 12) & 0x1FF) as usize
    }
}

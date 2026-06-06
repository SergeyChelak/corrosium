use crate::address::PhysicalAddress;

pub const PAGE_TABLE_SIZE: usize = 512;

#[derive(Clone)]
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_TABLE_SIZE],
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            entries: [PageTableEntry::empty(); PAGE_TABLE_SIZE],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    const fn set_bit(self, bit: u8, set: bool) -> Self {
        let bit_value = (set as u64) << bit;
        Self((self.0 & !(1 << bit)) | bit_value)
    }

    /// Set the Present (P) flag.
    pub const fn present(self) -> Self {
        self.set_bit(0, true)
    }

    /// Clear the Present (P) flag.
    pub const fn clear_present(self) -> Self {
        self.set_bit(0, false)
    }

    /// Set the Read/Write (R/W) flag.
    pub const fn writable(self) -> Self {
        self.set_bit(1, true)
    }

    /// Clear the Read/Write (R/W) flag.
    pub const fn clear_writable(self) -> Self {
        self.set_bit(1, false)
    }

    /// Set the User/Supervisor (U/S) flag.
    pub const fn user_accessible(self) -> Self {
        self.set_bit(2, true)
    }

    /// Clear the User/Supervisor (U/S) flag.
    pub const fn clear_user_accessible(self) -> Self {
        self.set_bit(2, false)
    }

    /// Set the Page-level Write-Through (PWT) flag.
    pub const fn write_through(self) -> Self {
        self.set_bit(3, true)
    }

    /// Clear the Page-level Write-Through (PWT) flag.
    pub const fn clear_write_through(self) -> Self {
        self.set_bit(3, false)
    }

    /// Set the Page-level Cache Disable (PCD) flag.
    pub const fn cache_disable(self) -> Self {
        self.set_bit(4, true)
    }

    /// Clear the Page-level Cache Disable (PCD) flag.
    pub const fn clear_cache_disable(self) -> Self {
        self.set_bit(4, false)
    }

    /// Set the Accessed (A) flag.
    pub const fn accessed(self) -> Self {
        self.set_bit(5, true)
    }

    /// Clear the Accessed (A) flag.
    pub const fn clear_accessed(self) -> Self {
        self.set_bit(5, false)
    }

    /// Set the Dirty (D) flag.
    pub const fn dirty(self) -> Self {
        self.set_bit(6, true)
    }

    /// Clear the Dirty (D) flag.
    pub const fn clear_dirty(self) -> Self {
        self.set_bit(6, false)
    }

    /// Set the Page Size (PS) flag.
    pub const fn huge(self) -> Self {
        self.set_bit(7, true)
    }

    /// Clear the Page Size (PS) flag.
    pub const fn clear_huge(self) -> Self {
        self.set_bit(7, false)
    }

    /// Set the Global (G) flag.
    pub const fn global(self) -> Self {
        self.set_bit(8, true)
    }

    /// Clear the Global (G) flag.
    pub const fn clear_global(self) -> Self {
        self.set_bit(8, false)
    }

    /// Set the Execute-Disable (XD) flag.
    pub const fn no_execute(self) -> Self {
        self.set_bit(63, true)
    }

    /// Clear the Execute-Disable (XD) flag.
    pub const fn clear_no_execute(self) -> Self {
        self.set_bit(63, false)
    }

    /// Set the physical address.
    pub const fn address(self, addr: PhysicalAddress) -> Self {
        let addr_u64 = addr.as_u64();
        let addr_mask = 0x000F_FFFF_FFFF_F000;
        Self((self.0 & !addr_mask) | (addr_u64 & addr_mask))
    }
}

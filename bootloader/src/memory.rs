use bootinfo::{
    // MemoryMapEntry,
    address::{PhysicalAddress, VirtualAddress},
};
use uefi::{
    boot::{self, AllocateType, MemoryType},
    mem::memory_map::MemoryMap,
};

const PAGE_TABLE_SIZE: usize = 512;

pub const PAGE_PRESENT: u64 = 1 << 0;
pub const PAGE_WRITABLE: u64 = 1 << 1;

type PageEntry = u64;

#[repr(align(4096))]
struct PageTable {
    entries: [PageEntry; PAGE_TABLE_SIZE],
}

impl PageTable {
    fn allocate() -> uefi::Result<*mut u8> {
        let addr = boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1)?;
        Ok(addr.as_ptr())
    }

    fn nullify(&mut self) {
        self.entries.fill(0);
    }
}

pub struct KernelSpaceMemoryManager {
    pml4: &'static mut PageTable,
    _pdpt: &'static mut PageTable,
    _pd: &'static mut PageTable,
}

impl KernelSpaceMemoryManager {
    pub fn new() -> uefi::Result<Self> {
        let pml4_addr = PageTable::allocate()?;
        let pdpt_addr = PageTable::allocate()?;
        let pd_addr = PageTable::allocate()?;

        let pml4 = unsafe { &mut *(pml4_addr as *mut PageTable) };
        let pdpt = unsafe { &mut *(pdpt_addr as *mut PageTable) };
        let pd = unsafe { &mut *(pd_addr as *mut PageTable) };

        pml4.nullify();
        pdpt.nullify();
        pd.nullify();

        pml4.entries[511] = pdpt_addr as u64 | PAGE_PRESENT | PAGE_WRITABLE;
        pdpt.entries[510] = pd_addr as u64 | PAGE_PRESENT | PAGE_WRITABLE;

        let manager = Self {
            pml4,
            _pdpt: pdpt,
            _pd: pd,
        };
        Ok(manager)
    }

    fn get_or_allocate_table(entry: &mut PageEntry) -> uefi::Result<&'static mut PageTable> {
        if *entry & PAGE_PRESENT == 0 {
            let addr = PageTable::allocate()?;
            let table = unsafe { &mut *(addr as *mut PageTable) };
            table.nullify();
            *entry = (addr as u64) | PAGE_PRESENT | PAGE_WRITABLE;
            Ok(table)
        } else {
            let addr = (*entry & !0xFFF) as *mut PageTable;
            Ok(unsafe { &mut *addr })
        }
    }

    pub fn allocate_mapped(
        &mut self,
        start_virt_addr: VirtualAddress,
        count: usize,
        flags: u64,
    ) -> uefi::Result<PhysicalAddress> {
        // allocate physical pages
        let phys_start =
            uefi::boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, count)?;

        // map each allocated page at physical address to upper half with provided virtual address start
        self.map_region(
            start_virt_addr,
            PhysicalAddress::new(phys_start.as_ptr() as u64),
            count,
            flags,
        )?;

        Ok(PhysicalAddress::new(phys_start.as_ptr() as u64))
    }

    // map a region of memory from physical address to virtual address with provided flags
    pub fn map_region(
        &mut self,
        virt_start: VirtualAddress,
        phys_start: PhysicalAddress,
        page_count: usize,
        flags: u64,
    ) -> uefi::Result<()> {
        for i in 0..page_count {
            let phys_page = phys_start + (i as u64 * 4096);
            let virt_page = virt_start + (i as u64 * 4096);

            let pdpt = Self::get_or_allocate_table(&mut self.pml4.entries[virt_page.pml4_index()])?;
            let pd = Self::get_or_allocate_table(&mut pdpt.entries[virt_page.pdpt_index()])?;
            let pt = Self::get_or_allocate_table(&mut pd.entries[virt_page.pd_index()])?;

            pt.entries[virt_page.pt_index()] = phys_page.as_u64() | flags;
        }
        Ok(())
    }
}

// pub fn get_memory_map_and_exit_boot_services() -> uefi::Result<bootinfo::MemoryMapInfo> {
//     let memory_map_owned = uefi::boot::memory_map(MemoryType::LOADER_DATA)?;

//     // reserve extra 10 entries for the memory map header and potential changes
//     let count = 10 + memory_map_owned.entries().count();
//     let size = count * core::mem::size_of::<MemoryMapEntry>();

//     let buffer = boot::allocate_pool(MemoryType::LOADER_DATA, size)?;
//     let memory_map_ptr = buffer.as_ptr() as *mut MemoryMapEntry;

//     let final_memory_map =
//         unsafe { uefi::boot::exit_boot_services(MemoryType::LOADER_DATA.into()) };

//     let final_count = final_memory_map.entries().count();
//     let memory_map_slice =
//         unsafe { core::slice::from_raw_parts_mut::<MemoryMapEntry>(memory_map_ptr, final_count) };

//     for (entry, descriptor) in memory_map_slice.iter_mut().zip(final_memory_map.entries()) {
//         entry.att = descriptor.att.bits();
//         entry.ty = descriptor.ty.0;
//         entry.phys_start = descriptor.phys_start;
//         entry.virt_start = descriptor.virt_start;
//         entry.page_count = descriptor.page_count;
//     }

//     Ok(bootinfo::MemoryMapInfo {
//         entries: memory_map_ptr,
//         count: final_count,
//     })
// }

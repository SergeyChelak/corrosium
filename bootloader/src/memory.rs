use bootinfo::MemoryMapEntry;
use uefi::{
    boot::{self, MemoryType},
    mem::memory_map::MemoryMap,
};

pub fn get_memory_map_and_exit_boot_services() -> uefi::Result<bootinfo::MemoryMapInfo> {
    let memory_map_owned = uefi::boot::memory_map(MemoryType::LOADER_DATA)?;

    // reserve extra 10 entries for the memory map header and potential changes
    let count = 10 + memory_map_owned.entries().count();
    let size = count * core::mem::size_of::<MemoryMapEntry>();

    let buffer = boot::allocate_pool(MemoryType::LOADER_DATA, size)?;
    let memory_map_ptr = buffer.as_ptr() as *mut MemoryMapEntry;

    let final_memory_map =
        unsafe { uefi::boot::exit_boot_services(MemoryType::LOADER_DATA.into()) };

    let final_count = final_memory_map.entries().count();
    let memory_map_slice =
        unsafe { core::slice::from_raw_parts_mut::<MemoryMapEntry>(memory_map_ptr, final_count) };

    for (entry, descriptor) in memory_map_slice.iter_mut().zip(final_memory_map.entries()) {
        entry.att = descriptor.att.bits();
        entry.ty = descriptor.ty.0;
        entry.phys_start = descriptor.phys_start;
        entry.virt_start = descriptor.virt_start;
        entry.page_count = descriptor.page_count;
    }

    Ok(bootinfo::MemoryMapInfo {
        entries: memory_map_ptr,
        count: final_count,
    })
}

#![no_main]
#![no_std]

use core::hint::black_box;

use uefi::Result;
use uefi::boot::{self, MemoryType};
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

use log::{debug, info, trace, warn};
use uefi::prelude::*;

struct MemoryMapEntry {
    memory_type: uefi::boot::MemoryType,
    base: usize,
    pages: usize,
    attributes: uefi::boot::MemoryAttribute,
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    if let Err(err) = uefi::boot::set_watchdog_timer(0, 0, None) {
        warn!("Failed to disable watchdog timer");
    }
    info!("UEFI App");
    load_kernel();
    // memory_map()ex;
    info!("Press any key...");
    wait_for_key();
    Status::SUCCESS
}

fn load_kernel() {
    debug!("Loading kernel");
    let Ok(loaded_image) = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())
    else {
        panic!("Failed to retrieve LoadedImage");
    };

    let device_handle = loaded_image.device().unwrap();

    let Ok(mut fs_protocol) = boot::open_protocol_exclusive::<SimpleFileSystem>(device_handle)
    else {
        panic!("Failed to retrieve SimpleFileSystem protocol");
    };

    let Ok(mut dir) = fs_protocol.open_volume() else {
        panic!("Failed to open volume");
    };
    let filename = cstr16!("\\EFI\\BOOT\\kernel.elf");
    let Ok(info) = dir.open(filename, FileMode::Read, FileAttribute::empty()) else {
        panic!("Failed to open kernel file");
    };
    info!("Kernel file info: {:?}", info);
}

fn memory_map() {
    let Ok(memory_map_owned) = uefi::boot::memory_map(MemoryType::RUNTIME_SERVICES_DATA) else {
        trace!("Failed to retrieve memory map");
        return;
    };
    debug!("Memory map:");
    for descriptor in memory_map_owned.entries() {
        info!(
            "Pages {}, ps: {}, type {:?}",
            descriptor.page_count, descriptor.phys_start, descriptor.ty
        );
    }

    let entries_count = memory_map_owned.entries().len();
    let size = entries_count * core::mem::size_of::<MemoryMapEntry>();
    let Ok(buffer) = boot::allocate_pool(boot::MemoryType::RUNTIME_SERVICES_DATA, size) else {
        trace!("Failed to allocate memory of size {}", size);
        return;
    };

    let mem_ptr = buffer.as_ptr() as *mut MemoryMapEntry;

    let mem_entries =
        unsafe { core::slice::from_raw_parts_mut::<MemoryMapEntry>(mem_ptr, entries_count) };

    for (entry, descriptor) in mem_entries.iter_mut().zip(memory_map_owned.entries()) {
        entry.attributes = descriptor.att;
        entry.base = descriptor.phys_start as usize;
        entry.pages = descriptor.page_count as usize;
        entry.memory_type = descriptor.ty;
    }
}

fn wait_for_key() {
    uefi::system::with_stdin(|stdin| {
        let Ok(key_event) = stdin.wait_for_key_event() else {
            return;
        };
        _ = uefi::boot::wait_for_event(&mut [key_event]);
        while let Ok(Some(key)) = stdin.read_key() {
            black_box(key);
        }
    });
}

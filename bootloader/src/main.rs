#![no_std]
#![no_main]
use core::hint::black_box;

use bootinfo::{MemoryMapInfo, *};
use log::{error, info};
use uefi::{
    boot::MemoryType,
    mem::memory_map::MemoryMap,
    prelude::*,
    proto::console::gop::{GraphicsOutput, PixelFormat},
    table::cfg::ConfigTableEntry,
};

// TODO: load from config file
const GRAPHICS_WIDTH: usize = 1920;
const GRAPHICS_HEIGHT: usize = 1080;

#[entry]
fn main() -> Status {
    let Ok(_) = uefi::helpers::init() else {
        return Status::NOT_READY;
    };

    info!("Corrosium Bootloader started");

    // Disable the UEFI watchdog timer
    // Setting timeout to 0 disables it.
    if let Err(e) = uefi::boot::set_watchdog_timer(0, 0x10000, None) {
        info!("Failed to disable watchdog timer: {:?}", e);
    }

    let Ok((acpi, smbios)) = get_rsdp() else {
        error!("Failed to fetch APIC/SMBIOS tables");
        return Status::ABORTED;
    };

    let Ok(framebuffer) = frame_buffer(GRAPHICS_WIDTH, GRAPHICS_HEIGHT) else {
        error!("Failed to setup graphics mode");
        return Status::ABORTED;
    };

    let Ok(memory_map) = get_memory_map() else {
        error!("Failed to make memory map");
        return Status::ABORTED;
    };

    let boot_info = BootInfo {
        acpi: acpi.0,
        smbios: smbios.0,
        framebuffer,
        memory_map,
    };

    info!("Press any key...");
    wait_for_key();

    Status::SUCCESS
}

struct ACPITable(ConfigTable);
struct SMBIOSTable(ConfigTable);

fn get_rsdp() -> uefi::Result<(ACPITable, SMBIOSTable)> {
    let mut acpi_table: Option<ConfigTable> = None;
    let mut smbios_table: Option<ConfigTable> = None;

    uefi::system::with_config_table(|entry| {
        for cfg in entry {
            if let Some(acpi) = acpi_config_table(cfg) {
                if is_newer_table(&acpi, &acpi_table) {
                    acpi_table = Some(acpi);
                }
                continue;
            }

            if let Some(smbios) = smbios_config_table(cfg) {
                if is_newer_table(&smbios, &smbios_table) {
                    smbios_table = Some(smbios);
                }
                continue;
            }
        }
    });

    let (Some(acpi), Some(smbios)) = (acpi_table, smbios_table) else {
        return Err(uefi::Status::NOT_FOUND.into());
    };

    Ok((ACPITable(acpi), SMBIOSTable(smbios)))
}

fn get_memory_map() -> uefi::Result<MemoryMapInfo> {
    let memory_map_owned = uefi::boot::memory_map(MemoryType::LOADER_DATA)?;

    // reserve extra 5 entries for the memory map header
    let count = 5 + memory_map_owned.entries().count();
    let size = count * core::mem::size_of::<MemoryMapEntry>();

    let buffer = boot::allocate_pool(MemoryType::LOADER_DATA, size)?;

    let memory_map_ptr = buffer.as_ptr() as *mut MemoryMapEntry;
    let memory_map_slice =
        unsafe { core::slice::from_raw_parts_mut::<MemoryMapEntry>(memory_map_ptr, count) };

    for (entry, descriptor) in memory_map_slice.iter_mut().zip(memory_map_owned.entries()) {
        entry.att = descriptor.att.bits();
        entry.ty = descriptor.ty.0;
        entry.phys_start = descriptor.phys_start;
        entry.virt_start = descriptor.virt_start;
        entry.page_count = descriptor.page_count;
    }

    let memory_map_info = bootinfo::MemoryMapInfo {
        entries: memory_map_ptr,
        count,
    };

    Ok(memory_map_info)
}

fn frame_buffer(target_width: usize, target_height: usize) -> uefi::Result<FrameBuffer> {
    let handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut protocol = boot::open_protocol_exclusive::<GraphicsOutput>(handle)?;

    let Some(mode) = protocol
        .modes()
        .into_iter()
        .filter(|mode| {
            matches!(
                mode.info().pixel_format(),
                PixelFormat::Rgb | PixelFormat::Bgr
            )
        })
        .map(|mode| {
            let (width, height) = mode.info().resolution();
            let sqr_diff = width.abs_diff(target_width) + height.abs_diff(target_height);
            (mode, sqr_diff)
        })
        .min_by_key(|(_, sqr_diff)| *sqr_diff)
        .map(|(mode, _)| mode)
    else {
        return Err(uefi::Status::NOT_FOUND.into());
    };

    protocol.set_mode(&mode)?;

    let (width, height) = mode.info().resolution();
    let stride = mode.info().stride();
    let mut fb = protocol.frame_buffer();
    let base_address = fb.as_mut_ptr() as *mut core::ffi::c_void;
    let size = fb.size();
    let pixel_format = match mode.info().pixel_format() {
        PixelFormat::Rgb => bootinfo::PixelFormat::RGB,
        PixelFormat::Bgr => bootinfo::PixelFormat::BGR,
        _ => return Err(uefi::Status::UNSUPPORTED.into()),
    };

    let frame_buffer = FrameBuffer {
        width,
        height,
        stride,
        base_address,
        size,
        pixel_format,
    };
    Ok(frame_buffer)
}

fn acpi_config_table(entry: &ConfigTableEntry) -> Option<ConfigTable> {
    let Some((address, version)) = (match entry.guid {
        ConfigTableEntry::ACPI2_GUID => Some((entry.address, 2)),
        ConfigTableEntry::ACPI_GUID => Some((entry.address, 1)),
        _ => None,
    }) else {
        return None;
    };
    let acpi = ConfigTable { address, version };
    Some(acpi)
}

fn smbios_config_table(entry: &ConfigTableEntry) -> Option<ConfigTable> {
    let Some((address, version)) = (match entry.guid {
        ConfigTableEntry::SMBIOS3_GUID => Some((entry.address, 3)),
        ConfigTableEntry::SMBIOS_GUID => Some((entry.address, 1)),
        _ => None,
    }) else {
        return None;
    };
    let smbios = ConfigTable { address, version };
    Some(smbios)
}

fn is_newer_table(candidate: &ConfigTable, other: &Option<ConfigTable>) -> bool {
    let Some(other) = other else {
        return true;
    };
    candidate.version > other.version
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

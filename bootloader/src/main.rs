#![no_std]
#![no_main]
use core::hint::black_box;

use bootinfo::*;
use log::{error, info};
use uefi::{
    boot::ScopedProtocol,
    prelude::*,
    proto::console::gop::{GraphicsOutput, PixelFormat},
    table::cfg::ConfigTableEntry,
};

mod config_table_holder;

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

    let mut boot_info = BootInfo::default();

    // Iterate across the Config Tables and enumerate them, displaying them to the log
    uefi::system::with_config_table(|entry| {
        for cfg in entry {
            if let Some(acpi) = acpi_config_table(cfg) {
                boot_info.set_acpi(acpi);
                info!("Did set ACPI config table");
                continue;
            }

            if let Some(smbios) = smbios_config_table(cfg) {
                boot_info.set_smbios(smbios);
                info!("Did set SMBIOS config table");
                continue;
            }

            // let holder = ConfigTableEntryHolder(cfg);
            // info!("Skipped table for {}", holder);
        }
    });

    if let Ok(fb) = frame_buffer(GRAPHICS_WIDTH, GRAPHICS_WIDTH) {
        boot_info.frame_buffer = Some(fb);
    } else {
        error!("Failed to setup graphics mode");
    }

    info!("Press any key...");
    wait_for_key();

    Status::SUCCESS
}

fn frame_buffer(target_width: usize, target_height: usize) -> uefi::Result<FrameBuffer> {
    let mut protocol = get_gfx_protocol()?;

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
            let sqr_diff = width.abs_diff(target_width) * height.abs_diff(target_height);
            (mode, sqr_diff)
        })
        .min_by_key(|(_, sqr_diff)| *sqr_diff)
        .map(|(mode, _)| mode)
    else {
        return Err(uefi::Status::NOT_FOUND.into());
    };

    protocol.set_mode(&mode)?;

    // TODO: provide pixel format

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

fn get_gfx_protocol() -> uefi::Result<ScopedProtocol<GraphicsOutput>> {
    let handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>()?;
    boot::open_protocol_exclusive::<GraphicsOutput>(handle)
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

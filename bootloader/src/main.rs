#![no_std]
#![no_main]
use core::hint::black_box;

use bootinfo::*;
use log::{debug, error, info};
use uefi::{
    boot::ScopedProtocol,
    prelude::*,
    proto::{
        console::gop::{GraphicsOutput, PixelFormat},
        device_path::hardware,
    },
    table::cfg::ConfigTableEntry,
};

mod config_table_holder;
use config_table_holder::*;

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

    draw_square();

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
    let frame_buffer = FrameBuffer { width, height };
    Ok(frame_buffer)
}

fn draw_square() {
    let mut gfx = get_gfx_protocol().unwrap();

    let mode_info = gfx.current_mode_info();

    // Lets draw a 2x2 square on the screen
    let top = 100 * mode_info.stride();
    let left = 100;
    let mut fb = gfx.frame_buffer();
    const WHITE_PIXEL: [u8; 3] = [0xffu8, 0xffu8, 0xffu8];
    unsafe {
        for i in 0..100 {
            // Draw the pixels of the top side
            fb.write_value((top + left + i) * 4, WHITE_PIXEL);

            // Draw the pixels of the left vertical side
            fb.write_value((top + left + i * mode_info.stride()) * 4, WHITE_PIXEL);

            // Draw the pixels of the bottom side
            fb.write_value((top + left + 100 * mode_info.stride() + i) * 4, WHITE_PIXEL);

            // Draw the pixels of the right vertical side
            fb.write_value((top + left + i * mode_info.stride() + 100) * 4, WHITE_PIXEL);
        }
    };
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

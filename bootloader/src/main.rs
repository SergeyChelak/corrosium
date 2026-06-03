#![no_std]
#![no_main]

use bootinfo::*;
use log::{debug, error, info};
use uefi::{ prelude::*};

mod rsdp;
use rsdp::*;

mod utils;
use utils::*;

mod gop;
use gop::*;

mod memory;
use memory::*;

mod kernel_loader;
use kernel_loader::*;

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

    let Ok(kernel_info) = load_kernel() else {
        error!("Failed to load kernel");
        return Status::LOAD_ERROR;
    };
    debug!(
        "Kernel loaded. Entry point: {:#X}, Range: {:#X} - {:#X}",
        kernel_info.entry_point, kernel_info.start_addr, kernel_info.end_addr
    );

    let Ok(rsdp) = RSDP::setup() else {
        error!("Failed to fetch APIC/SMBIOS tables");
        return Status::ABORTED;
    };
    debug!("acpi/smbios tables loaded");
    wait_for_key();

    let Ok(framebuffer) = GOP::default().framebuffer_info() else {
        error!("Failed to setup graphics mode");
        return Status::ABORTED;
    };

    let Ok(memory_map) = get_memory_map_and_exit_boot_services() else {
        error!("Failed to exit boot services and get memory map");
        return Status::ABORTED;
    };

    let boot_info = BootInfo {
        acpi: rsdp.acpi,
        smbios: rsdp.smbios,
        framebuffer,
        memory_map,
        kernel_range: KernelRange {
            start_addr: kernel_info.start_addr,
            end_addr: kernel_info.end_addr,
        },
    };

    let entry_point: extern "sysv64" fn(&BootInfo) -> ! =
        unsafe { core::mem::transmute(kernel_info.entry_point as usize) };

    entry_point(&boot_info);
}

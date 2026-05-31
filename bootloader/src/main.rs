#![no_std]
#![no_main]
use core::hint::black_box;

use bootinfo::*;
use log::info;
use uefi::{prelude::*, table::cfg::ConfigTableEntry};

mod config_table_holder;
use config_table_holder::*;

#[entry]
fn main() -> Status {
    let Ok(_) = uefi::helpers::init() else {
        return Status::NOT_READY;
    };

    info!("Corrosium Bootloader started");

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

            let holder = ConfigTableEntryHolder(cfg);
            info!("Skipped table for {}", holder);
        }
    });

    info!("Press any key...");
    wait_for_key();

    Status::SUCCESS
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

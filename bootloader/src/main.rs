#![no_std]
#![no_main]

use core::time::Duration;

use bootinfo::*;
use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    let Ok(_) = uefi::helpers::init() else {
        return Status::NOT_READY;
    };

    info!("Corrosium Bootloader started");

    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}

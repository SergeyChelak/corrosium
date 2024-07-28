use core::arch::asm;

use crate::asm86::{in_b, out_b};

const PRIMARY_DRIVE: u16 = 0x1f0;

const REG_DATA: u16 = 0; // data read/write
const REG_SEC_COUNT: u16 = 2; // number of sectors
const REG_LBA_LOW: u16 = 3; // LBA low
const REG_LBA_MID: u16 = 4; // LBA mid
const REG_LBA_HIGH: u16 = 5; // LBA high
const REG_DRIVE: u16 = 6; // select drive
const REG_CMD_STAT: u16 = 7; // command/status

pub fn load(lba: usize, sectors: u8, target: *mut usize) {
    ata_load(PRIMARY_DRIVE, lba, sectors, target)
}

#[inline(never)]
fn ata_load(drive_port: u16, lba: usize, sectors: u8, target: *mut usize) {
    unsafe {
        asm!("pusha", "mov edi, {0}", in(reg) target);
    }
    // highest 8 bit of LBA | master
    out_b(drive_port + REG_DRIVE, (lba >> 24 & 0xff) as u8 | 0xe0);
    // number of sectors
    out_b(drive_port + REG_SEC_COUNT, sectors & 0xff);
    // LBA
    out_b(drive_port + REG_LBA_LOW, ((lba >> 0) & 0xff) as u8);
    out_b(drive_port + REG_LBA_MID, ((lba >> 8) & 0xff) as u8);
    out_b(drive_port + REG_LBA_HIGH, ((lba >> 16) & 0xff) as u8);
    // send read sectors command
    out_b(drive_port + REG_CMD_STAT, 0x20);
    for _ in 0..sectors {
        // retry
        while in_b(drive_port + REG_CMD_STAT) & 8 == 0 {
            io_delay(1)
        }
        // read data into buffer
        unsafe {
            asm!(
                "push ecx",
                "mov ecx, 256",
                "rep insw",
                "pop ecx",
                in("edx") drive_port + REG_DATA,
            )
        }
    }
    unsafe { asm!("popa") };
}

#[inline]
fn io_delay(times: u32) {
    (0..times).for_each(|_| out_b(0x80, 0))
}

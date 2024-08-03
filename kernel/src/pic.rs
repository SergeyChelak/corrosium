/// 8259 PIC
use crate::x86::{self, inb, io_wait, outb};

/// IO base address for master PIC
const PIC1: x86::PortNumber = 0x20;
/// IO base address for slave PIC
const PIC2: x86::PortNumber = 0xA0;
const PIC1_COMMAND: x86::PortNumber = PIC1;
const PIC1_DATA: x86::PortNumber = PIC1 + 1;
const PIC2_COMMAND: x86::PortNumber = PIC2;
const PIC2_DATA: x86::PortNumber = PIC2 + 1;

// Indicates that ICW4 will be present
const ICW1_ICW4: u8 = 0x01;
// Initialization - required!
const ICW1_INIT: u8 = 0x10;
// 8086/88 (MCS-80/85) mode
const ICW4_8086: u8 = 0x01;

// End-of-interrupt command code
const PIC_EOI: u8 = 0x20;

pub fn remap() {
    remap_with_offsets(0x20, 0x28)
}

fn remap_with_offsets(master_offset: u8, slave_offset: u8) {
    // save masks
    let master_mask = inb(PIC1_DATA);
    let slave_mask = inb(PIC2_DATA);

    // starts the initialization sequence (in cascade mode)
    outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);

    io_wait();
    outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();
    // ICW2: Master PIC vector offset
    outb(PIC1_DATA, master_offset);
    io_wait();
    // ICW2: Slave PIC vector offset
    outb(PIC2_DATA, slave_offset);
    io_wait();
    // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
    outb(PIC1_DATA, 4);
    io_wait();
    // ICW3: tell Slave PIC its cascade identity (0000 0010)
    outb(PIC2_DATA, 2);
    io_wait();

    // ICW4: have the PICs use 8086 mode (and not 8080 mode)
    outb(PIC1_DATA, ICW4_8086);
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();

    // restore masks
    outb(PIC1_DATA, master_mask); // restore saved masks.
    outb(PIC2_DATA, slave_mask);
}

pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        outb(PIC2_COMMAND, PIC_EOI)
    }
    outb(PIC1_COMMAND, PIC_EOI);
}

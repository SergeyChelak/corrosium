use core::{arch::asm, mem};

const TOTAL_INTERRUPTS: usize = 512;

const KERNEL_CODE_SELECTOR: u16 = 0x8;
const KERNEL_DATA_SELECTOR: u16 = 0x10;

const DESCRIPTOR_TYPE_INTERRUPT_GATE_32: u8 = 0xe;
const DESCRIPTOR_TYPE_TRAP_GATE_32: u8 = 0xf;

pub fn idt_init() {
    (0..TOTAL_INTERRUPTS).for_each(|i| {
        idt_add(i, default_interrupt_handler as *const usize);
    });
}

pub fn idt_add(interrupt: usize, handler: *const usize) -> bool {
    unsafe { IDT.add(interrupt, handler) }
}

pub fn idt_load() {
    unsafe { IDT.load() }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct InterruptDescriptor {
    offset_low: u16,     // offset bits 0..15
    selector: u16,       // a code segment selector in GDT or LDT
    zero: u8,            // unused, set to 0
    type_attributes: u8, // gate type, dpl, and p fields
    offset_high: u16,    // offset bits 16..31
}

impl InterruptDescriptor {
    fn set(&mut self, handler: *const usize) {
        let address = handler as usize;
        self.offset_low = (address & 0xffff) as u16;
        self.offset_high = (address >> 16) as u16;
        // set present bit
        self.type_attributes |= 1 << 7;
    }
}

static INTERRUPT_DESCRIPTOR: InterruptDescriptor = {
    let selector: u16 = {
        // A Segment Selector with multiple fields which must point to a valid code segment in GDT
        KERNEL_CODE_SELECTOR
    };

    let type_attributes: u8 = {
        let gate_type: u8 = DESCRIPTOR_TYPE_INTERRUPT_GATE_32;
        let storage: u8 = 0 << 3;
        let dpl = 3 << 5; // ring 3

        gate_type | storage | dpl
    };

    InterruptDescriptor {
        offset_low: 0,
        selector,
        zero: 0,
        type_attributes,
        offset_high: 0,
    }
};

#[repr(C, packed)]
struct IdtRegister {
    limit: u16,                            // table size minus one
    base: *const InterruptDescriptorTable, // base address of start of IDT
}

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable {
    entries: [INTERRUPT_DESCRIPTOR; TOTAL_INTERRUPTS],
};

#[repr(C, packed)]
struct InterruptDescriptorTable {
    entries: [InterruptDescriptor; TOTAL_INTERRUPTS],
}

impl InterruptDescriptorTable {
    pub fn load(&self) {
        let idt_register = IdtRegister {
            limit: (self.entries.len() * mem::size_of::<InterruptDescriptor>() - 1) as u16,
            base: self,
        };
        unsafe { asm!("lidt [{0:e}]", in(reg) &idt_register) }
    }

    pub fn add(&mut self, interrupt: usize, handler: *const usize) -> bool {
        let Some(interrupt) = self.entries.get_mut(interrupt) else {
            return false;
        };
        interrupt.set(handler);
        true
    }
}

extern "x86-interrupt" fn default_interrupt_handler() {
    // no op
}

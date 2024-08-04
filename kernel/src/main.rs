#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use vga_buffer::*;

mod idt;
mod pic;

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn kernel_main() -> ! {
    main()
}

fn main() -> ! {
    setup_registers();
    pic::remap();
    setup_interrupts();
    print_intro();
    system_halt()
}

fn setup_registers() {
    unsafe {
        core::arch::asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "mov ebp, 0x300000",
            "mov esp, ebp",
        )
    }
}

fn print_intro() {
    // vga_text::clear();
    (0..4).for_each(|_| println!());
    println!("kernel started");
}

fn setup_interrupts() {
    use idt::*;
    idt_init();
    (0x20..0x28).for_each(|i| {
        idt_add(i, master_no_interrupt_handler as *const usize);
    });
    (0x28..0x30).for_each(|i| {
        idt_add(i, slave_no_interrupt_handler as *const usize);
    });
    idt_add(0, div_by_zero_handler as *const usize);
    idt_add(0x21, interrupt_handler as *const usize);
    idt_load();
    arch_x86::sti();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    system_halt()
}

fn system_halt() -> ! {
    println!("halt system");
    use arch_x86::*;
    // spin_forever()
    loop {
        hlt();
    }
}

extern "x86-interrupt" fn div_by_zero_handler() {
    println!("int 0: divide by zero");
}

extern "x86-interrupt" fn interrupt_handler() {
    println!("key pressed");
    pic::send_eoi(0);
}

extern "x86-interrupt" fn master_no_interrupt_handler() {
    pic::send_eoi(0);
}

extern "x86-interrupt" fn slave_no_interrupt_handler() {
    pic::send_eoi(8);
}

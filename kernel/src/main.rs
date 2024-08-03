#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use x86::{insb, insw, outb, outw};

mod idt;
mod vga_text;
mod x86;

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn kernel_main() -> ! {
    main()
}

fn main() -> ! {
    setup_registers();
    print_intro();
    setup_interrupts();
    ports();
    system_halt()
}

fn ports() {
    let port = 0x60;

    let initial = insb(port);
    outb(port, 0x1);
    let val = insb(port);
    println!("BYTE: initial {:b}, updated {:b}", initial, val);

    outw(port, 3);
    let initial = insw(port);
    outw(port, 500);
    let val = insw(port);
    println!("WORD: initial {:}, updated {:}", initial, val);

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
    idt_add(0, div_by_zero_handler as *const usize);
    idt_load();
    unsafe { core::arch::asm!("sti") }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    system_halt()
}

fn system_halt() -> ! {
    println!("halt system");
    x86::cli();
    loop {
        x86::hlt();
    }
}

static mut CALL_COUNT: usize = 0;

extern "x86-interrupt" fn div_by_zero_handler() {
    unsafe {
        if CALL_COUNT > 5 {
            return;
        }
        CALL_COUNT += 1;
        println!("int 0: divide by zero, called {} times", CALL_COUNT);
    }
}

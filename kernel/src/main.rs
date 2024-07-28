#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

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
    throw_error();
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
    idt_add(0, div_by_zero_handler as *const usize);
    idt_load();
    unsafe { core::arch::asm!("sti") }
}

fn throw_error() {
    println!("try to throw interrupt...");
    unsafe { core::arch::asm!("int 0") }
    // unsafe { core::arch::asm!("mov eax, 0", "div eax") }
    // {
    //     let mut x = 10;
    //     let y = x / x;
    //     x /= y - 1;
    //     println!("x = {x}");
    // }
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

#![no_std]
#![no_main]

mod vga_text;
mod x86;

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn kernel_main() -> ! {
    vga_text::clear();
    println!("kernel_main()");
    system_halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    system_halt()
}

fn system_halt() -> ! {
    x86::cli();
    loop {
        x86::hlt();
    }
}

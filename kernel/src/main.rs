#![no_std]
#![no_main]

use core::arch::asm;

use bootinfo::*;

#[unsafe(no_mangle)]
pub extern "sysv64" fn _start(info: &BootInfo) -> ! {
    // draw_square(&boot_info.framebuffer);
    loop {
        unsafe {
            asm!("cli", "hlt");
        }
    }
}
/*
fn draw_square(framebuffer: &FrameBufferInfo) {
    // Lets draw a 2x2 square on the screen
    let top = 100 * framebuffer.stride;
    let left = 100;
    let fb = framebuffer.base_address as *mut u8;
    const WHITE_PIXEL: [u8; 3] = [0xffu8, 0xffu8, 0xffu8];
    unsafe {
        for i in 0..100 {
            // Draw the pixels of the top side
            core::ptr::write_volatile(fb.add((top + left + i) * 4) as *mut [u8; 3], WHITE_PIXEL);

            // Draw the pixels of the left vertical side
            core::ptr::write_volatile(
                fb.add((top + left + i * framebuffer.stride) * 4) as *mut [u8; 3],
                WHITE_PIXEL,
            );

            // Draw the pixels of the bottom side
            core::ptr::write_volatile(
                fb.add((top + left + 100 * framebuffer.stride + i) * 4) as *mut [u8; 3],
                WHITE_PIXEL,
            );

            // Draw the pixels of the right vertical side
            core::ptr::write_volatile(
                fb.add((top + left + i * framebuffer.stride + 100) * 4) as *mut [u8; 3],
                WHITE_PIXEL,
            );
        }
    };
}
 */

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // no op
    }
}

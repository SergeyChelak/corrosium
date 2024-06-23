use core::arch::asm;

pub fn print(message: &str) {
    unsafe {
        asm!(
            "mov si, {0:x}",
            "mov bx, 0",
            "2:",
            "lodsb",
            "cmp al, 0",
            "je 1f",
            "mov ah, 0xe",
            "int 0x10",
            "jmp 2b",
            "1:",
            in(reg) message.as_ptr()
        )
    }
}

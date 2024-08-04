#![no_std]
pub fn sum_mod(a: u32, b: u32, module: u32) -> u32 {
    if b == 0 {
        return a;
    }
    let b = module - b;
    if a >= b {
        return a - b;
    }
    module - b + a
}

pub fn checksum(address: usize, count: usize) -> u32 {
    let mut sum = 0;
    for i in 0..count {
        let addr = address + i;
        let byte: u8 = unsafe { core::ptr::read(addr as *const _) };
        sum = sum_mod(sum, byte as u32, u32::MAX);
    }
    sum
}

fn sum_mod(a: u32, b: u32, module: u32) -> u32 {
    if b == 0 {
        return a;
    }
    let b = module - b;
    if a >= b {
        return a - b;
    }
    module - b + a
}

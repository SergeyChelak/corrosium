use gp_utils::sum_mod;

pub fn checksum(address: usize, count: usize) -> u32 {
    let mut sum = 0;
    for i in 0..count {
        let addr = address + i;
        let byte: u8 = unsafe { core::ptr::read(addr as *const _) };
        sum = sum_mod(sum, byte as u32, u32::MAX);
    }
    sum
}

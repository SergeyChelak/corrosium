use std::env;
use std::fs::*;
use std::io::Read;

use gp_utils::sum_mod;

const BUFFER_SIZE: usize = 512;

fn calc_sum(path: &str) -> std::io::Result<u32> {
    let mut file = File::open(path)?;
    let mut sum = 0u32;
    loop {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let bytes = file.read(&mut buffer)?;
        if bytes == 0 {
            break;
        }
        for byte in buffer.iter().take(bytes).map(|x| *x as u32) {
            sum = sum_mod(sum, byte, u32::MAX);
        }
    }
    Ok(sum)
}

fn main() {
    let Some(path) = env::args().nth(1) else {
        println!("no parameters");
        return;
    };
    let Ok(sum) = calc_sum(&path) else {
        println!("io error ocurred");
        return;
    };
    println!("Control sum: {sum}");
}

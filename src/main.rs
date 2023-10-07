use std::time::Instant;
use std::hint::black_box;

const LENGTH_MASK: u8 = 192;
const MAX_SIZE: usize = 24;

fn main() {
    let start = Instant::now();
    for _ in 0..1_000_000 {
        for i in u8::MIN..u8::MAX {
            let j = len2(i);
            black_box(j);
        }
    }
    let benchmark_shift = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        for i in u8::MIN..u8::MAX {
            let j = len(i);
            black_box(j);
        }
    }
    let benchmark_cmp = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        for i in u8::MIN..u8::MAX {
            let j = len3(i);
            black_box(j);
        }
    }
    let benchmark_branch = start.elapsed();

    println!("cmp: {benchmark_cmp:?}");
    println!("shift: {benchmark_shift:?}");
    println!("branch: {benchmark_branch:?}");
}

#[inline(always)]
pub fn len(b: u8) -> usize {
    (b as usize)
        .wrapping_sub(LENGTH_MASK as usize)
        .min(MAX_SIZE)
}

#[inline(always)]
pub fn len2(b: u8) -> usize {
    let mask = ((b as i8).wrapping_shr(8)) as u8;
    let len = (!mask & (b.wrapping_sub(LENGTH_MASK))) | (mask & 24);
    len as usize
}

#[inline(always)]
pub fn len3(b: u8) -> usize {
    let mut len = b.wrapping_sub(192) as usize;
    if b < LENGTH_MASK {
        len = MAX_SIZE;
    }
    len
}

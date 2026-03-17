use std::io::{self, BufRead};

type Int64 = i64;

fn init_bit(bit: &mut Vec<i32>) {
    let n = bit[0] as usize;
    for i in 1..=n {
        bit[i] = 0;
    }
}

fn inc(bit: &mut Vec<i32>, x: usize) {
    let n = bit[0] as usize;
    let mut idx = x;
    while idx <= n {
        bit[idx] += 1;
        idx += idx & (idx as isize).wrapping_neg() as usize; // Using bit manipulation to find the next index
    }
}

fn get_sum(bit: &Vec<i32>, x: usize) -> i32 {
    let mut res = 0;
    let mut idx = x;
    while idx > 0 {
        res += bit[idx];
        idx -= idx & (idx as isize).wrapping_neg() as usize; // Using bit manipulation to find the previous index
    }
    res
}

fn count(m: i32, a: &Vec<i32>, n: usize) -> Int64 {
    let mut bit = vec![0; 2 * n + 2]; // +2 to account for 1-based indexing
    bit[0] = (2 * n + 1) as i32;
    init_bit(&mut bit);
    
    let mut cnt: Int64 = 0;
    let mut sum = n as i32 + 1;
    inc(&mut bit, sum as usize);
    
    for &value in a.iter() {
        sum += if value < m { -1 } else { 1 };
        cnt += get_sum(&bit, sum as usize) as Int64;
        inc(&mut bit, sum as usize);
    }
    
    cnt
}

fn run() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }
    
    let mut l = 1;
    let mut r = 1_000_000_001; // 1 billion + 1
    while r - l > 1 {
        let m = (l + r) / 2;
        let cnt = count(m, &a, n);
        if 4 * cnt >= (n as Int64) * (n as Int64 + 1) {
            l = m;
        } else {
            r = m;
        }
    }
    
    println!("{}", l);
}

fn main() {
    run();
}

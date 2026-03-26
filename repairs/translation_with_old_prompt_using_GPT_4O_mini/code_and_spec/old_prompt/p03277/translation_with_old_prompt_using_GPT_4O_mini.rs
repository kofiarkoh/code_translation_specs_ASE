use std::io::{self, BufRead};

type Int64 = i64;

struct FenwickTree {
    bit: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        let mut bit = vec![0; 2 * size + 2];
        bit[0] = (2 * size + 1) as i32; // Invariant: first element holds the value 2*n + 1
        FenwickTree { bit }
    }

    fn inc(&mut self, x: usize) {
        let n = self.bit[0] as usize;
        let mut idx = x;
        while idx <= n {
            self.bit[idx] += 1;
            idx += idx & (idx as isize).wrapping_neg() as usize; // x += x & -x
        }
    }

    fn get_sum(&self, x: usize) -> i32 {
        let mut res = 0;
        let mut idx = x;
        while idx > 0 {
            res += self.bit[idx];
            idx -= idx & (idx as isize).wrapping_neg() as usize; // x -= x & -x
        }
        res
    }
}

fn count(m: i32, a: &[i32]) -> Int64 {
    let n = a.len();
    let mut bit = FenwickTree::new(n);
    let mut cnt = 0;
    let mut sum = n as i32 + 1;
    bit.inc(sum as usize); // Initialize Fenwick Tree with the first value

    for &value in a {
        sum += if value < m { -1 } else { 1 };
        cnt += bit.get_sum(sum as usize) as Int64;
        bit.inc(sum as usize);
    }
    cnt
}

fn run() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse n");
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().expect("Failed to parse array element");
        a.push(value);
    }

    let mut l = 1;
    let mut r = 1_000_000_001; // 1000000000 + 1

    while r - l > 1 {
        let m = (l + r) / 2;
        let cnt = count(m, &a);
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

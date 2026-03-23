use std::io::{self, Read};

const TABAICHO_SIZE: usize = 8667;
const TABAICHO_BLOCK: i64 = 1_000_000_000;

struct Tabaicho {
    a: [i64; TABAICHO_SIZE], // little-endian base 1e9 blocks
}

impl Tabaicho {
    fn new() -> Self {
        Self { a: [0; TABAICHO_SIZE] }
    }

    fn set(&mut self, num: i32) {
        self.a[0] = num as i64;
        for i in 1..TABAICHO_SIZE {
            self.a[i] = 0;
        }
    }

    fn mul(&mut self, num: i32) {
        let mut carry: i64 = 0;
        let n = num as i64;
        for i in 0..TABAICHO_SIZE {
            let mut tmp = self.a[i] * n + carry;
            self.a[i] = tmp % TABAICHO_BLOCK;
            carry = tmp / TABAICHO_BLOCK;
        }
    }

    fn div(&mut self, num: i32) {
        let mut rem: i64 = 0;
        let d = num as i64;
        for i in (0..TABAICHO_SIZE).rev() {
            rem = rem * TABAICHO_BLOCK + self.a[i];
            self.a[i] = rem / d;
            rem %= d;
        }
    }

    fn to_decimal_string(&self) -> String {
        // Match C behavior: find highest non-zero block, print it without padding,
        // then print remaining blocks with %09d.
        let mut i = TABAICHO_SIZE;
        while i > 0 && self.a[i - 1] == 0 {
            i -= 1;
        }
        // In the original C, if all blocks are zero, it would access out of bounds.
        // Under the spec, this situation is not expected for the executed paths.
        // Still, we handle it safely.
        if i == 0 {
            return "0".to_string();
        }

        let mut s = String::new();
        s.push_str(&format!("{}", self.a[i - 1]));
        for idx in (0..i - 1).rev() {
            s.push_str(&format!("{:09}", self.a[idx]));
        }
        s
    }
}

fn main() {
    // Read entire stdin and parse as C scanf("%d%d%d") would for three ints.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();
    let mut r: i32 = it.next().unwrap().parse().unwrap();

    // Spec: r' = r - n*m using C int arithmetic; overflow is UB in C.
    // In Rust, we use wrapping ops to avoid panics; if overflow would have occurred,
    // the C behavior is unspecified anyway.
    r = r.wrapping_sub(n.wrapping_mul(m));

    if r < 0 {
        print!("0\n");
    } else if r == 0 {
        print!("1\n");
    } else {
        // Compute (n + r - 1)C(n - 1) using fixed-capacity big integer blocks.
        let mut big = Tabaicho::new();
        big.set(1);

        let mut n2: i32 = n - 1;
        let total: i32 = n.wrapping_add(r).wrapping_sub(1); // n + r - 1
        let other: i32 = total.wrapping_sub(n - 1); // (n+r-1)-(n-1) == r
        if other < n - 1 {
            n2 = other;
        }

        let mut j: i32 = total;
        for _ in 0..n2 {
            big.mul(j);
            j = j.wrapping_sub(1);
        }
        for i in 1..=n2 {
            big.div(i);
        }

        let out = big.to_decimal_string();
        print!("{out}\n");
    }
}

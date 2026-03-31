use std::io::{self, Read};

const BASE: u32 = 1_000_000_000;
const BASE_U64: u64 = 1_000_000_000;

// Fixed-capacity big-integer in base 1e9 blocks, least-significant block first.
// Capacity is intentionally fixed (like the C program); overflow is not detected.
const LIMBS: usize = 512;

#[derive(Clone)]
struct BigInt {
    a: [u32; LIMBS],
}

impl BigInt {
    fn new() -> Self {
        Self { a: [0u32; LIMBS] }
    }

    fn set_u32(&mut self, v: u32) {
        self.a = [0u32; LIMBS];
        self.a[0] = v;
    }

    fn mul_small(&mut self, x: u32) {
        let mut carry: u64 = 0;
        let x64 = x as u64;
        for i in 0..LIMBS {
            let cur = self.a[i] as u64 * x64 + carry;
            self.a[i] = (cur % BASE_U64) as u32;
            carry = cur / BASE_U64;
        }
        // Any remaining carry is silently dropped (fixed-capacity semantics).
    }

    fn div_small(&mut self, d: u32) {
        // Division by zero is not checked (matches "no explicit validation").
        let d64 = d as u64;
        let mut rem: u64 = 0;
        for i in (0..LIMBS).rev() {
            let cur = rem * BASE_U64 + self.a[i] as u64;
            self.a[i] = (cur / d64) as u32;
            rem = cur % d64;
        }
        // Remainder is discarded; under preconditions division is exact.
    }

    fn to_decimal_string(&self) -> String {
        let mut i = LIMBS;
        while i > 0 && self.a[i - 1] == 0 {
            i -= 1;
        }
        if i == 0 {
            return "0\n".to_string();
        }
        let mut s = String::new();
        s.push_str(&format!("{}", self.a[i - 1]));
        if i >= 2 {
            for j in (0..i - 1).rev() {
                s.push_str(&format!("{:09}", self.a[j]));
            }
        }
        s.push('\n');
        s
    }
}

fn main() {
    // Read entire stdin and parse three signed decimal integers like scanf("%d%d%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();
    let r: i32 = it.next().unwrap().parse().unwrap();

    // Compute r' = r - n*m using native int arithmetic.
    // In C, overflow here is undefined; in Rust we use wrapping to avoid panics and
    // to emulate typical two's-complement behavior as closely as possible.
    let rp: i32 = r.wrapping_sub(n.wrapping_mul(m));

    if rp < 0 {
        print!("0\n");
        return;
    }
    if rp == 0 {
        print!("1\n");
        return;
    }

    // For rp > 0, compute C(n + rp - 1, n - 1) using multiply-descend then divide 1..k,
    // where k = min(n-1, rp).
    let nn = n as i64;
    let rr = rp as i64;

    let k: i64 = {
        let n1 = nn - 1;
        if n1 < rr { n1 } else { rr }
    };

    let mut big = BigInt::new();
    big.set_u32(1);

    // Multiply by descending integers: (n + rp - 1) * (n + rp - 2) * ... for k terms.
    // Then divide by 1..k.
    let mut x: i64 = nn + rr - 1;
    for _ in 0..k {
        // Under preconditions, x fits in u32 for the original program's loop variable usage.
        big.mul_small(x as u32);
        x -= 1;
    }
    for d in 1..=k {
        big.div_small(d as u32);
    }

    print!("{}", big.to_decimal_string());
}

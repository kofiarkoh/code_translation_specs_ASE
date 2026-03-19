use std::io::{self, Read};

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn modpow(mut a: i64, mut n: i64, modu: i64) -> i64 {
    let mut ans: i64 = 1;
    while n > 0 {
        if (n & 1) != 0 {
            ans = (ans * a) % modu;
        }
        a = (a * a) % modu;
        n /= 2;
    }
    ans
}

fn modinv(a: i64, modu: i64) -> i64 {
    modpow(a, modu - 2, modu)
}

fn max_i32(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn chmax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

fn chmin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C uses a VLA `int h[n+10];` and initializes all to 0.
    let mut h = vec![0i32; n + 10];
    for i in 0..n {
        if let Some(v) = it.next() {
            h[i] = v.parse().unwrap();
        }
    }

    // Note: C code shadows the `max` function with a local variable named `max`.
    let mut max_val: i32 = 0;
    let mut count: i32 = 0;

    // In C: for (int i=0; i<n-1; i++) { ... }
    // If n is 0, `n-1` underflows in unsigned Rust; emulate C's behavior by
    // only iterating when n >= 2 (since for n<=1 the loop runs 0 times in C).
    if n >= 2 {
        for i in 0..(n - 1) {
            if h[i + 1] <= h[i] {
                count += 1;
            } else {
                count = 0;
            }
            chmax(&mut max_val, count);
        }
    }

    print!("{}\n", max_val);
}

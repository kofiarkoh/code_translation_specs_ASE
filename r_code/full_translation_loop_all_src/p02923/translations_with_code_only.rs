use std::io::{self, Read};

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    let mut c: i32;
    while b > 0 {
        c = a % b;
        a = b;
        b = c;
    }
    a
}

fn modpow(mut a: i64, mut n: i64, modu: i64) -> i64 {
    let mut ans: i64 = 1;
    while n > 0 {
        if (n & 1) != 0 {
            ans = ans * a % modu;
        }
        a = a * a % modu;
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
    // Read all input, parse as integers like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C uses VLA int h[n+10]; and initializes all to 0.
    // If n is negative, C behavior is undefined; we emulate a reasonable behavior by
    // treating the length as 0 to avoid panics.
    let n_usize = if n > 0 { n as usize } else { 0usize };
    let mut h = vec![0i32; n_usize + 10];

    for i in 0..n_usize {
        if let Some(v) = it.next() {
            h[i] = v.parse().unwrap();
        } else {
            // If input is short, leave remaining as 0 (already initialized),
            // similar to uninitialized reads not occurring in the C code.
            break;
        }
    }

    let mut maxv: i32 = 0;
    let mut count: i32 = 0;

    // for(int i=0;i<n-1;i++) in C:
    // if n <= 1, loop doesn't run.
    if n_usize >= 2 {
        for i in 0..(n_usize - 1) {
            if h[i + 1] <= h[i] {
                count += 1;
            } else {
                count = 0;
            }
            chmax(&mut maxv, count);
        }
    }

    print!("{}\n", maxv);
}

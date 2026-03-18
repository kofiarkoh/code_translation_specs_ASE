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
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => return,
    };

    // C uses a VLA int h[n+10]; and initializes all to 0.
    let mut h = vec![0i32; n + 10];

    for i in 0..n {
        if let Some(s) = it.next() {
            h[i] = s.parse::<i32>().unwrap();
        }
    }

    // Note: C code shadows the max() function with a local variable named `max`.
    let mut max_val: i32 = 0;
    let mut count: i32 = 0;

    // for (i=0; i<n-1; i++) ... ; if n==0, C loop doesn't run.
    // In Rust, avoid underflow by guarding.
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

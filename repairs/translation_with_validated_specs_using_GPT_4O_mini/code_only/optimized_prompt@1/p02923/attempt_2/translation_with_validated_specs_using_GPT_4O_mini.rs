use std::io::{self, BufRead};

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

fn modpow(mut a: i64, mut n: i64, mod_val: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 != 0 {
            ans = ans * a % mod_val;
        }
        a = a * a % mod_val;
        n /= 2;
    }
    ans
}

fn modinv(a: i64, mod_val: i64) -> i64 {
    modpow(a, mod_val - 2, mod_val)
}

fn max(a: i32, b: i32) -> i32 {
    if a < b {
        b
    } else {
        a
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
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
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line and parse it as usize
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");

    // Initialize the array with zeros
    let mut h = vec![0; n + 10];

    // Read the next n lines and parse them as i32
    for i in 0..n {
        h[i] = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse height");
    }

    let mut max_count = 0;
    let mut count = 0;

    // Calculate the maximum count of non-increasing elements
    for i in 0..n - 1 {
        if h[i + 1] <= h[i] {
            count += 1;
        } else {
            count = 0;
        }
        chmax(&mut max_count, count);
    }

    // Output the result
    println!("{}", max_count);
}

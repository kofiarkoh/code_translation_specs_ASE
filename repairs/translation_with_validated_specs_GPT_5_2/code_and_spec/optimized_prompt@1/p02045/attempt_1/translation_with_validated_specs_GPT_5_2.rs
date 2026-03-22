use std::io::{self, Read};
use std::process;

fn rect_dist(a: i32, b: i32, c: i32, d: i32, x: i32, y: i32, is_src: bool) -> i32 {
    let mut cost: i32 = 0;
    if x < a {
        cost += a - x;
    }
    if c < x {
        cost += x - c;
    }
    if y < b {
        cost += b - y;
    }
    if d < y {
        cost += y - d;
    }
    if is_src && cost > 0 {
        cost -= 1; // escape discount, never below 0 because cost>0
    }
    cost
}

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        // If N cannot be read (EOF), terminate normally with status 0.
        let n_opt = it.next();
        let n_str = match n_opt {
            None => return,
            Some(s) => s,
        };
        let n: i32 = match n_str.parse() {
            Ok(v) => v,
            Err(_) => return, // Not specified; treat as no further datasets.
        };

        // If non-positive N is read, stop processing and exit normally.
        if n <= 0 {
            return;
        }

        // For a started dataset, missing required values => exit status 1.
        let a: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let b: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let c: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let d: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };

        // C code uses fixed arrays of size 128; spec says N <= 127.
        // We keep the same effective constraint by allocating N+1 points.
        let n_usize = n as usize;
        let mut x: Vec<i32> = Vec::with_capacity(n_usize + 1);
        let mut y: Vec<i32> = Vec::with_capacity(n_usize + 1);

        for _ in 0..=n_usize {
            let xi: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => process::exit(1),
            };
            let yi: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => process::exit(1),
            };
            x.push(xi);
            y.push(yi);
        }

        let mut total: i32 = 0;
        for i in 1..=n_usize {
            let direct = (x[i - 1] - x[i]).abs() + (y[i - 1] - y[i]).abs();
            let via = rect_dist(a, b, c, d, x[i - 1], y[i - 1], true)
                + rect_dist(a, b, c, d, x[i], y[i], false);
            total += if direct < via { direct } else { via };
        }

        println!("{total}");
    }
}

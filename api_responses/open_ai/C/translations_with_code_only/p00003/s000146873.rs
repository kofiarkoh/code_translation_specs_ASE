use std::io::{self, Read};

fn main() {
    // Read all input tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Parse N as 32-bit signed int to match typical C 'int'
    let n: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };

    // We use i32 and explicit wrapping arithmetic for multiplications/additions.
    // In C, signed overflow is undefined, but on common platforms it wraps (two's complement).
    // To emulate common practical behavior and avoid Rust panics in debug builds, use wrapping ops.
    let mut i: i32 = 0;
    while i < n {
        let a: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => break,
        };
        let b: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => break,
        };
        let c: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => break,
        };

        let mut max: i32 = 0;

        if a <= b && b <= c {
            max = c;
            if c.wrapping_mul(c) == a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b)) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if a <= c && c <= b {
            max = b;
            if b.wrapping_mul(b) == a.wrapping_mul(a).wrapping_add(c.wrapping_mul(c)) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if b <= c && c <= a {
            max = a;
            if a.wrapping_mul(a) == b.wrapping_mul(b).wrapping_add(c.wrapping_mul(c)) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if b <= a && a <= c {
            max = c;
            if c.wrapping_mul(c) == a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b)) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if c <= a && a <= b {
            max = b;
            if b.wrapping_mul(b) == a.wrapping_mul(a).wrapping_add(c.wrapping_mul(c)) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if c <= b && b <= a {
            max = a;
            if a.wrapping_mul(a) == b.wrapping_mul(b).wrapping_add(c.wrapping_mul(c)) {
                println!("YES");
            } else {
                println!("NO");
            }
        }

        // 'max' is assigned like in the C code (not further used).
        let _ = max;

        i += 1;
    }
}

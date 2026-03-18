use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    // If N is missing or unparsable, treat as no valid input -> output 0.
    let n: usize = match it.next().and_then(|s| s.parse::<isize>().ok()) {
        Some(v) if v >= 0 => v as usize,
        _ => {
            println!("0");
            return;
        }
    };

    // Fixed memory limit: accommodate up to 1000 integers.
    // If n > 1000, we only store/process the first 1000 values (narrowest interpretation
    // consistent with "fixed memory limit for storing input integers").
    let cap = n.min(1000);

    let mut a: Vec<i64> = Vec::with_capacity(cap);
    for _ in 0..cap {
        match it.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => a.push(v),
            None => break, // Missing/invalid numbers: process what we have.
        }
    }

    // If fewer than 3 integers are available (or n < 3), output zero.
    if n < 3 || a.len() < 3 {
        println!("0");
        return;
    }

    let mut count: i64 = 0;
    for i in 0..=(a.len() - 3) {
        let x = a[i];
        let y = a[i + 1];
        let z = a[i + 2];

        if (x < y && y < z) || (x > y && y > z) {
            count += 1;
        }
    }

    println!("{}", count);
}

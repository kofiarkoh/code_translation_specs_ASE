use std::io::{self, BufRead};

const P: i64 = 998244353;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or_else(|_| {
            eprintln!("Invalid number of elements");
            std::process::exit(1);
        }),
        _ => {
            eprintln!("Failed to read the number of elements");
            std::process::exit(1);
        }
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);

    // Read the elements
    for _ in 0..n {
        let value: i64 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or_else(|_| {
                eprintln!("Invalid integer value");
                std::process::exit(1);
            }),
            _ => {
                eprintln!("Failed to read an integer value");
                std::process::exit(1);
            }
        };
        a.push(value);
    }

    // Sort the array
    a.sort_unstable();

    // Compute the product of GCDs
    let mut ans: i64 = 1;
    for (i, &value) in a.iter().enumerate() {
        ans = (ans * gcd(value, i as i64)) % P;
    }

    // Output the result
    println!("{}", ans);
}

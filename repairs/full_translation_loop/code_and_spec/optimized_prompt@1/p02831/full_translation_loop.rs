use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut inputs = handle.lines().filter_map(Result::ok);

    // Read the two long integers from input
    let a: i64 = match inputs.next() {
        Some(line) => line.trim().parse().unwrap_or_else(|_| {
            eprintln!("Invalid input for a");
            std::process::exit(1);
        }),
        None => {
            eprintln!("Expected first input");
            std::process::exit(1);
        }
    };

    let b: i64 = match inputs.next() {
        Some(line) => line.trim().parse().unwrap_or_else(|_| {
            eprintln!("Invalid input for b");
            std::process::exit(1);
        }),
        None => {
            eprintln!("Expected second input");
            std::process::exit(1);
        }
    };

    // Preconditions check
    if a < 0 {
        eprintln!("First input must be a non-negative integer.");
        return;
    }
    if b <= 0 {
        eprintln!("Second input must be a positive integer.");
        return;
    }

    // Find the smallest multiple of `a` that is also a multiple of `b`
    for i in 1..=b + 1 {
        let multiple = a * i;
        if multiple % b == 0 {
            println!("{}", multiple);
            break;
        }
    }
}

use std::io::{self, BufRead};

fn souti(a: char, b: char) -> char {
    if a == 'T' && b == 'F' {
        'F'
    } else {
        'T'
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of boolean characters
    let mut lines = reader.lines();
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or_else(|_| {
            eprintln!("Invalid input format.");
            std::process::exit(1);
        }),
        _ => {
            eprintln!("Failed to read the number of characters.");
            std::process::exit(1);
        }
    };

    // Preconditions check
    if n < 2 {
        eprintln!("The number of boolean characters must be at least 2.");
        std::process::exit(1);
    }

    let mut p = Vec::with_capacity(n);
    
    // Read the boolean characters
    for _ in 0..n {
        let line = match lines.next() {
            Some(Ok(line)) => line.trim(),
            _ => {
                eprintln!("Failed to read the expected number of characters.");
                std::process::exit(1);
            }
        };

        if line.len() != 1 || (line != "T" && line != "F") {
            eprintln!("Invalid character. Only 'T' and 'F' are allowed.");
            std::process::exit(1);
        }

        p.push(line.chars().next().unwrap());
    }

    // Evaluate the logical expression
    let mut result = souti(p[0], p[1]);
    for &value in &p[2..] {
        result = souti(result, value);
    }

    // Output the result
    println!("{}", result);
}

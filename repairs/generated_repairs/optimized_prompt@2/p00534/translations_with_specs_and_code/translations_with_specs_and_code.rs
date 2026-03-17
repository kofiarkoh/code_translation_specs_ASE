use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>());
    let n = nums.next().unwrap_or_else(|_| {
        eprintln!("Failed to parse n");
        std::process::exit(1);
    });
    let m = nums.next().unwrap_or_else(|_| {
        eprintln!("Failed to parse m");
        std::process::exit(1);
    });

    // Preconditions check
    assert!(n >= 1, "n must be >= 1");
    assert!(m >= 1, "m must be >= 1");

    // Read the sequences
    let mut k = vec![0; n + 1]; // Using 1-based indexing
    let mut t = vec![0; m + 1]; // Using 1-based indexing

    for i in 1..=n {
        k[i] = lines.next().unwrap().unwrap().parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Failed to parse k[{}]", i);
            std::process::exit(1);
        });
    }

    for i in 1..=m {
        t[i] = lines.next().unwrap().unwrap().parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Failed to parse t[{}]", i);
            std::process::exit(1);
        });
    }

    // Initialize the cost matrix
    let mut hi = vec![vec![i32::MAX; n + 1]; m + 1];

    // Set the first column to zero
    for i in 0..=m {
        hi[i][0] = 0;
    }

    // Compute the minimum cost
    for i in 1..=m {
        for j in 1..=n {
            hi[i][j] = k[j] * t[i] + hi[i - 1][j - 1];
            if hi[i][j] > hi[i - 1][j] {
                hi[i][j] = hi[i - 1][j];
            }
        }
    }

    // Output the result
    println!("{}", hi[m][n]);
}

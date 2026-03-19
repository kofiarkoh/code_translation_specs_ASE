use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    
    // Parse n and k with error handling
    let n: usize = parts.next().unwrap().parse().unwrap_or_else(|_| {
        eprintln!("Invalid input for n");
        std::process::exit(1);
    });
    let k: i32 = parts.next().unwrap().parse().unwrap_or_else(|_| {
        eprintln!("Invalid input for k");
        std::process::exit(1);
    });

    // Preconditions check
    if n < 1 || n > 100_000 || k < 0 {
        // Undefined behavior as per specification; we do not handle this explicitly.
        return;
    }

    // Read the array of integers with error handling
    let mut array: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().parse().unwrap_or_else(|_| {
            eprintln!("Invalid input for array element");
            std::process::exit(1);
        }))
        .collect();

    // Check the condition based on the number of elements
    if n == k as usize { // Convert k to usize for comparison
        // Check if all elements satisfy the modular condition
        let all_satisfy = array.iter().all(|&x| x % k == 0);
        // Mark processed elements
        for x in &mut array {
            if *x % k == 0 {
                *x = -1; // Mark as processed
            }
        }
        // Output result
        if all_satisfy {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        // Evaluate the parity of the count of processed elements
        let processed_count = array.iter().filter(|&&x| x == -1).count();
        // Output "Yes" if processed count is even, "No" if odd
        if processed_count % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

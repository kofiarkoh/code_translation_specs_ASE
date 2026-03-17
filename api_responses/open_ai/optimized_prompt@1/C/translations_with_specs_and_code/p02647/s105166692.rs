use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N and K
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the sequence A
    let mut a = vec![0; n + 1]; // Using 1-based indexing
    let second_line = lines.next().unwrap().unwrap();
    let a_parts: Vec<usize> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    for i in 1..=n {
        a[i] = a_parts[i - 1]; // Fill A with input values
    }

    let mut min: usize;
    let mut b = vec![0; n + 2]; // B array for processing

    for _ in 1..=k {
        min = n; // Reset min for each iteration
        b.fill(0); // Reset B array

        for i in 1..=n {
            if a[i] >= i {
                b[1] += 1; // Increment B[1]
            } else {
                b[i - a[i]] += 1; // Increment B[i - A[i]]
            }
            if a[i] >= n - i {
                b[n + 1] -= 1; // Decrement B[N + 1]
            } else {
                b[i + a[i] + 1] -= 1; // Decrement B[i + A[i] + 1]
            }
        }

        let mut tmp = 0;
        for i in 1..=n {
            tmp += b[i]; // Cumulative sum
            a[i] = tmp; // Update A[i]
            if a[i] < min {
                min = a[i]; // Update min
            }
        }
    }

    // Output the result
    for i in 1..n {
        print!("{} ", a[i]);
    }
    println!("{}", a[n]); // Print last element with newline
}

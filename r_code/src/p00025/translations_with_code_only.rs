use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut A = [0; 4];
    let mut B = [0; 4];

    loop {
        // Read the first array A
        if input.read_line(&mut String::new()).is_err() {
            break; // Break on EOF or read error
        }
        let mut line = String::new();
        if input.read_line(&mut line).is_err() {
            break; // Break on EOF or read error
        }
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() < 4 {
            break; // Ensure we have enough numbers
        }
        A.copy_from_slice(&nums[0..4]);

        // Read the second array B
        line.clear();
        if input.read_line(&mut line).is_err() {
            break; // Break on EOF or read error
        }
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() < 4 {
            break; // Ensure we have enough numbers
        }
        B.copy_from_slice(&nums[0..4]);

        let mut hit = 0;
        let mut blow = 0;

        for a in 0..4 {
            for b in 0..4 {
                if A[a] == B[b] {
                    if a == b {
                        hit += 1;
                    } else {
                        blow += 1;
                    }
                }
            }
        }

        // Print the results
        println!("{} {}", hit, blow);
    }
}

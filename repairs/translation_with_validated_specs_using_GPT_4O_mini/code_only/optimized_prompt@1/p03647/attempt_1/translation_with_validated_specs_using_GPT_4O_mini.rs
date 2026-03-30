use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get n
    let first_line = lines.next().unwrap().unwrap();
    let mut n: i32 = first_line.trim().parse().unwrap();
    
    // Initialize a pointer-like structure
    let mut p = 0;
    let mut m = vec![0; 1 << 18]; // Equivalent to m[1<<18] in C

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        if nums.len() < 2 {
            break; // If we don't have two numbers, exit the loop
        }
        
        let a = nums[0];
        let b = nums[1];

        // Emulate the behavior of the C code
        if p != 0 && n != 0 {
            n = 0;
        } else {
            p += 1;
        }

        if a != 1 {
            if b != n {
                p = 0;
            } else {
                p = a;
            }
        } else {
            p = b;
        }
    }

    // Output the result
    if n != 0 {
        println!("IMPOSSIBLE");
    } else {
        println!("POSSIBLE");
    }
}

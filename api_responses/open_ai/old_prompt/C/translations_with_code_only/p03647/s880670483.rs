fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get n
    let first_line = lines.next().unwrap().unwrap();
    let mut n: i32 = first_line.trim().parse().unwrap();
    
    // Initialize a pointer-like structure
    let mut p: usize = 0; // This will act like a pointer index
    let m = vec![0; 1 << 18]; // Simulating the array m[1<<18]

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        if nums.len() < 2 {
            break; // If we don't have enough numbers, exit the loop
        }
        
        let a = nums[0];
        let b = nums[1];

        // Emulating the pointer behavior
        if p < m.len() && m[p] != 0 && n != 0 {
            n = 0;
        } else {
            p += 1; // Increment the pointer index
        }

        // Update p based on the conditions
        if a != 1 {
            if b != n {
                p = 0; // Reset p if b != n
            } else {
                p = a as usize; // Set p to a
            }
        } else {
            p = b as usize; // Set p to b
        }
    }

    // Output the result
    if n != 0 {
        println!("IMPOSSIBLE");
    } else {
        println!("POSSIBLE");
    }
}

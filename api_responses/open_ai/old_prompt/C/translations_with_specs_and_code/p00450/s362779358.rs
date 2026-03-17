use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut go = vec![0; 100_000]; // Initialize a vector with a capacity of 100,000

    for line in stdin.lock().lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break; // Terminate if n is zero
        }

        // Read the sequence of integers into the go array
        for i in 0..n {
            go[i] = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
        }

        // Process the go array according to the specified rules
        for i in (1..n).step_by(2) {
            if go[i - 1] != go[i] {
                let mut j = i as isize - 1; // Start from the previous index
                while j >= 0 && go[j as usize] != go[i] {
                    go[j as usize] = go[i]; // Modify the go array
                    j -= 1; // Move to the previous index
                }
            }
        }

        // Count the number of zeros in the modified go array
        let cnt = go.iter().take(n).filter(|&&x| x == 0).count();
        println!("{}", cnt); // Output the count of zeros
    }
}

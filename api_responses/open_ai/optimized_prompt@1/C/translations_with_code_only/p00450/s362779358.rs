use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut go = vec![0; 100_000]; // Create a vector to hold the integers

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap(); // Read and parse the integer

        if n == 0 {
            break; // Exit if n is 0
        }

        // Read n integers into the go array
        for i in 0..n {
            let value: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
            go[i as usize] = value;
        }

        // Process the array as per the C code logic
        for i in (1..n).step_by(2) {
            if go[i as usize - 1] != go[i as usize] {
                let mut j = i - 1;
                while j >= 0 && go[j as usize] != go[i as usize] {
                    go[j as usize] = go[i as usize];
                    if j == 0 { break; } // Prevent underflow
                    j -= 1;
                }
            }
        }

        // Count the number of zeros in the go array
        let cnt = (0..n).filter(|&i| go[i as usize] == 0).count();
        println!("{}", cnt); // Print the count
    }
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut go = vec![0; 100_000]; // Initialize the array with a size of 100,000

    for line in stdin.lock().lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break; // Terminate if n is zero
        }

        // Read the sequence of integers into the go array
        for i in 0..n {
            go[i] = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
        }

        // Process the array according to the specified rules
        for i in (1..n).step_by(2) {
            if go[i - 1] != go[i] {
                let mut j = i - 1;
                while j >= 0 && go[j] != go[i] {
                    go[j] = go[i];
                    if j == 0 { break; } // Prevent underflow
                    j -= 1;
                }
            }
        }

        // Count the number of zeros in the modified array
        let cnt = go.iter().take(n).filter(|&&x| x == 0).count();
        println!("{}", cnt);
    }
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.expect("Failed to read line");
        let mut parts = input.split_whitespace();
        
        // Read n and x from input
        let n: i32 = parts.next().unwrap().parse().expect("Invalid input for n");
        let x: i32 = parts.next().unwrap().parse().expect("Invalid input for x");

        // Termination condition
        if n == 0 && x == 0 {
            break;
        }

        // Count unique triplets (i, j, k) such that 1 <= i < j < k <= n and i + j + k == x
        let mut count = 0;

        // Iterate over all combinations of i, j, k
        for i in 1..=n {
            for j in (i + 1)..=n {
                for k in (j + 1)..=n {
                    if i + j + k == x {
                        count += 1;
                    }
                }
            }
        }

        // Output the count of valid triplets
        println!("{}", count);
    }
}

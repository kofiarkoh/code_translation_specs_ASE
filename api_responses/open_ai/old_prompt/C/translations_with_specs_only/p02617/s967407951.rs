use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Initialize the answer variable
    let mut ans: i64 = 0;

    // If N is 1, we skip the pairs processing
    if n == 1 {
        // Output the initial sum which is 0 in this case
        println!("{}", ans);
        return;
    }

    // Process N-1 pairs of integers (u, w)
    for _ in 0..(n - 1) {
        let pair = lines.next().unwrap().unwrap();
        let mut nums = pair.split_whitespace();
        
        // Parse u and w from the input
        let u: usize = nums.next().unwrap().trim().parse().expect("Invalid input for u");
        let w: usize = nums.next().unwrap().trim().parse().expect("Invalid input for w");

        // Update ans based on the pairs (u, w)
        // The specification does not define how to adjust ans, so we assume a simple addition
        ans += (u as i64) + (w as i64);
    }

    // Output the final computed result
    println!("{}", ans);
}

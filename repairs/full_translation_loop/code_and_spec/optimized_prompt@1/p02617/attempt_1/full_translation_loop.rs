use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of iterations N
    let N: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for N");

    // Initialize ans as a long long integer (i64 in Rust)
    let mut ans: i64 = 0;

    // Calculate the initial value of ans
    for i in 1..=N {
        ans += (N - i + 2) as i64 * (N - i + 1) as i64 / 2;
    }

    // Read N-1 pairs of integers (u, w)
    for _ in 1..N {
        let pair = lines.next().unwrap().unwrap();
        let mut nums = pair.split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid input for pair"));
        
        let mut u = nums.next().unwrap();
        let mut w = nums.next().unwrap();

        // Ensure u is less than or equal to w
        if u > w {
            std::mem::swap(&mut u, &mut w);
        }

        // Update ans based on the pair (u, w)
        ans -= (u as i64) * (N - w + 1) as i64;
    }

    // Output the result
    println!("{}", ans);
}

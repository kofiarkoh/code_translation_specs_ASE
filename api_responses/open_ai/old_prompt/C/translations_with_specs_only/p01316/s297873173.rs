fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().expect("Invalid input for n");
        if n == 0 {
            break; // Terminate if n is zero
        }

        let m: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for m");
        let c: Vec<i32> = lines.next().unwrap().unwrap().trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for adjustment values"))
            .collect();

        // Initialize the dp array
        let mut dp = vec![vec![-1; 256]; 20001];
        dp[0][128] = 0; // Set the initial state

        // Process the values
        for i in 0..n {
            let x: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for x");
            for j in 0..=255 {
                if dp[i][j] != -1 {
                    for &adjust in &c {
                        let new_j = (j as i32 + adjust + 128).clamp(0, 255) as usize; // Adjust and clamp
                        let new_cost = dp[i][j] + (x - (j as i32 - 128)).abs(); // Calculate cost
                        if dp[i + 1][new_j] == -1 || new_cost < dp[i + 1][new_j] {
                            dp[i + 1][new_j] = new_cost; // Update dp if new cost is lower
                        }
                    }
                }
            }
        }

        // Find the minimum cost in the last row of dp
        let min_cost = dp[n].iter().filter(|&&cost| cost != -1).min().unwrap_or(&i32::MAX);
        println!("{}", min_cost);
    }
}

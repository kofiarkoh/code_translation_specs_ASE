use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().expect("Invalid input for n");
        if n == 0 {
            break; // Terminate if n is zero
        }

        let m: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for m");
        let adjustments: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid adjustment value"))
            .collect();

        let mut dp = vec![vec![-1; 256]; 20001]; // Initialize dp array
        dp[0][128] = 0; // Set base case

        for i in 0..n {
            let x: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for x");
            for j in 0..=255 {
                if dp[i][j] != -1 {
                    for &c in &adjustments {
                        let new_j = j as i32 + c + x; // Calculate new state
                        if new_j >= 0 && new_j <= 255 {
                            let new_cost = dp[i][j] + c.abs(); // Update cost
                            if dp[i + 1][new_j as usize] == -1 || new_cost < dp[i + 1][new_j as usize] {
                                dp[i + 1][new_j as usize] = new_cost; // Store minimum cost
                            }
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

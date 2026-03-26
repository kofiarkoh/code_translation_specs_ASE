use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for n, x, y
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let x: usize = params.next().unwrap().parse().unwrap();
    let y: usize = params.next().unwrap().parse().unwrap();

    // Initialize the dynamic programming table
    // Using a 2D vector to represent dp array
    let mut dp = vec![vec![0; y + 1]; x + 1];

    // Process each item
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut item_params = line.split_whitespace();
        let a: usize = item_params.next().unwrap().parse().unwrap();
        let b: usize = item_params.next().unwrap().parse().unwrap();
        let c: usize = item_params.next().unwrap().parse().unwrap();
        let d: usize = item_params.next().unwrap().parse().unwrap();

        // Update the dp table in reverse order to avoid overwriting
        for i in (a..=x).rev() {
            for j in (b..=y).rev() {
                // Calculate the new value if this item is taken
                let new_value = dp[i - a][j] + d;
                // Update the dp table if the new value is greater
                if new_value > dp[i][j] {
                    dp[i][j] = new_value;
                }
                // Update the second resource gain
                if j + c <= y {
                    let new_value_with_gain = dp[i][j] + d;
                    if new_value_with_gain > dp[i][j + c] {
                        dp[i][j + c] = new_value_with_gain;
                    }
                }
            }
        }
    }

    // Find the maximum value obtainable
    let max_value = dp.iter().flat_map(|row| row.iter()).copied().max().unwrap_or(0);

    // Print the result
    println!("{}", max_value);
}

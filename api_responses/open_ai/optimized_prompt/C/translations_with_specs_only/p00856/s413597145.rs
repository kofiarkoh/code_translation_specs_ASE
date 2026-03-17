use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut inputs = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap());

        let n = inputs.next().unwrap();
        let t = inputs.next().unwrap();
        let l = inputs.next().unwrap();
        let b = inputs.next().unwrap();

        // Termination condition
        if n == 0 && t == 0 && l == 0 && b == 0 {
            break;
        }

        // Read special squares
        let l_squares: Vec<i32> = (0..l).map(|_| inputs.next().unwrap()).collect();
        let b_squares: Vec<i32> = (0..b).map(|_| inputs.next().unwrap()).collect();

        // Calculate the probability
        let probability = calculate_probability(n, t, &l_squares, &b_squares);
        println!("{:.6}", probability);
    }
}

// Function to calculate the probability of success
fn calculate_probability(n: i32, t: i32, l_squares: &[i32], b_squares: &[i32]) -> f64 {
    // Memoization array to store probabilities
    let mut memo = vec![vec![-1.0; (t + 1) as usize]; (n + 1) as usize];

    // Start the recursive probability calculation
    calculate_probability_recursive(n, t, 0, &l_squares, &b_squares, &mut memo)
}

// Recursive function to compute probabilities
fn calculate_probability_recursive(n: i32, t: i32, position: i32, l_squares: &[i32], b_squares: &[i32], memo: &mut Vec<Vec<f64>>) -> f64 {
    // Base case: if no turns left
    if t == 0 {
        return if position == n { 1.0 } else { 0.0 };
    }

    // Check memoization
    if memo[position as usize][t as usize] != -1.0 {
        return memo[position as usize][t as usize];
    }

    let mut probability = 0.0;

    // Move to the next position
    for step in 1..=2 {
        let next_position = position + step;

        // Check bounds
        if next_position <= n {
            // Check for special squares
            if l_squares.contains(&next_position) {
                probability += calculate_probability_recursive(n, t - 1, next_position, l_squares, b_squares, memo) * 1.5; // Example effect of 'L'
            } else if b_squares.contains(&next_position) {
                probability += calculate_probability_recursive(n, t - 1, next_position, l_squares, b_squares, memo) * 0.5; // Example effect of 'B'
            } else {
                probability += calculate_probability_recursive(n, t - 1, next_position, l_squares, b_squares, memo);
            }
        }
    }

    // Store the computed probability in memoization array
    memo[position as usize][t as usize] = probability;
    probability
}

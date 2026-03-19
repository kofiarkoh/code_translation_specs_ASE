use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let inputs: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check for termination condition
        if inputs.len() == 4 && inputs.iter().all(|&x| x == 0) {
            break;
        }

        // Validate input length and values
        if inputs.len() != 4 || inputs.iter().any(|&x| x < 0) {
            continue; // Skip invalid input
        }

        let (n, t, l, b) = (inputs[0], inputs[1], inputs[2], inputs[3]);

        // Check the sum of L and B does not exceed N
        if l + b > n {
            continue; // Skip invalid input
        }

        let l_positions = read_positions(l);
        let b_positions = read_positions(b);

        let probability = calculate_probability(n, t, &l_positions, &b_positions);
        println!("{:.6}", probability);
    }
}

// Function to read positions of obstacles
fn read_positions(count: i32) -> Vec<i32> {
    let mut positions = Vec::new();
    for _ in 0..count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(pos) = input.trim().parse::<i32>() {
            positions.push(pos);
        }
    }
    positions
}

// Function to calculate the probability of winning
fn calculate_probability(n: i32, t: i32, l_positions: &[i32], b_positions: &[i32]) -> f64 {
    // Initialize memoization array with 0.0 for the end position
    let mut memo = vec![0.0; (n + 1) as usize];
    memo[n as usize] = 1.0; // Reached the end

    // Dynamic programming to calculate probabilities
    for _ in 0..t {
        let mut new_memo = vec![0.0; (n + 1) as usize];
        for pos in 0..=n {
            if memo[pos as usize] > 0.0 {
                // Check possible moves
                for move_distance in 1..=6 {
                    let new_pos = pos + move_distance;
                    if new_pos <= n && !is_obstacle(new_pos, l_positions, b_positions) {
                        new_memo[new_pos as usize] += memo[pos as usize] / 6.0; // Distribute probability
                    }
                }
            }
        }
        memo = new_memo;
    }

    // Calculate final probability
    let final_probability = memo[n as usize];
    final_probability.min(1.0).max(0.0) // Ensure the probability is between 0.0 and 1.0
}

// Function to check if a position is an obstacle
fn is_obstacle(pos: i32, l_positions: &[i32], b_positions: &[i32]) -> bool {
    l_positions.contains(&pos) || b_positions.contains(&pos)
}

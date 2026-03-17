fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of sets
    let first_line = lines.next().unwrap().unwrap();
    let num_sets: usize = first_line.parse().unwrap();

    // Check for valid number of sets
    if num_sets == 0 || num_sets > 40_002 {
        println!("-1");
        return;
    }

    let mut total_score: i64 = 0; // Using i64 to accommodate long long integer results

    // Process each set of four integers
    for _ in 0..num_sets {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Ensure we have exactly four integers
        if values.len() != 4 {
            println!("-1");
            return;
        }

        let (health, attack, defense, score) = (values[0], values[1], values[2], values[3]);

        // Check for invalid states based on health and score thresholds
        if health < 0 || score < 0 {
            println!("-1");
            return;
        }

        // Example computation: accumulate score based on attack and defense
        // This is a placeholder for the actual game mechanics logic
        total_score += attack - defense;

        // If total_score goes negative, we consider it an invalid state
        if total_score < 0 {
            println!("-1");
            return;
        }
    }

    // Output the computed total score
    println!("{}", total_score);
}

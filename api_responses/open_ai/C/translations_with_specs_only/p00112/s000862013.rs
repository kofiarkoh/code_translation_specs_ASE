use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of subsequent durations
        if numbers.is_empty() || numbers[0] == 0 {
            break; // Termination condition
        }

        let count = numbers[0] as usize;
        if count > 10_000 || count + 1 != numbers.len() {
            continue; // Invalid series, skip processing
        }

        // Extract the time durations
        let durations = &numbers[1..=count];

        // Compute the weighted sum of waiting times
        let total_weighted_sum = compute_weighted_sum(durations);
        println!("{}", total_weighted_sum);
    }
}

fn compute_weighted_sum(durations: &[i64]) -> i64 {
    let mut total = 0;
    let mut cumulative_wait = 0;

    for &duration in durations {
        cumulative_wait += duration;
        total += cumulative_wait; // Accumulate waiting times
    }

    total
}

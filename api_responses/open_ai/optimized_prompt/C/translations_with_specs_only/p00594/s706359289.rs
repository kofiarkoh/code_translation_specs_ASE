use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of subsequent integers
        if numbers.is_empty() || numbers[0] == 0 {
            break; // Terminate on zero
        }

        let count = numbers.remove(0); // Remove the count from the list
        if count != numbers.len() as i32 {
            continue; // If the count does not match the number of integers, skip this sequence
        }

        let mut frequency = HashMap::new();
        for &num in &numbers {
            *frequency.entry(num).or_insert(0) += 1;
        }

        let half_count = count / 2;
        let mut most_frequent = None;
        let mut max_frequency = 0;

        for (&num, &freq) in &frequency {
            if freq > max_frequency {
                max_frequency = freq;
                most_frequent = Some(num);
            }
        }

        // Check if the most frequent integer's frequency exceeds half the total count
        if let Some(mf) = most_frequent {
            if max_frequency > half_count {
                println!("{}", mf);
            } else {
                println!("NO COLOR");
            }
        } else {
            println!("NO COLOR"); // Fallback if no frequent number found
        }
    }
}

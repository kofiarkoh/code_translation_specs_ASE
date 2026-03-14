use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Create a HashMap to store the counts of each unique integer
    let mut counts: HashMap<i32, usize> = HashMap::new();
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read pairs of integers from standard input until EOF
    for line in reader.lines() {
        if let Ok(line) = line {
            // Split the line by comma and parse the integers
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(first), Ok(second)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                    // Increment the count for each integer
                    *counts.entry(first).or_insert(0) += 1;
                    *counts.entry(second).or_insert(0) += 1;
                }
            }
        }
    }

    // Collect integers that have a count greater than one
    let mut output: Vec<(i32, usize)> = counts.into_iter()
        .filter(|&(_, count)| count > 1)
        .collect();

    // Sort the output by the integer value
    output.sort_by_key(|&(integer, _)| integer);

    // Print the results
    for (integer, count) in output {
        println!("{} {}", integer, count);
    }
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();

        // Read the first two integers which represent the sizes of the sequences
        let size1: usize = iter.next().unwrap_or("0").parse().expect("Invalid input");
        let size2: usize = iter.next().unwrap_or("0").parse().expect("Invalid input");

        // Termination condition
        if size1 == 0 {
            break;
        }

        // Initialize counts for the two sequences (1 to 20)
        let mut counts1 = [0; 20];
        let mut counts2 = [0; 20];

        // Read the first sequence
        for _ in 0..size1 {
            let value: usize = iter.next().unwrap_or("0").parse().expect("Invalid input");
            if value >= 1 && value <= 20 {
                counts1[value - 1] += 1; // Increment count for the value
            }
        }

        // Read the second sequence
        for _ in 0..size2 {
            let value: usize = iter.next().unwrap_or("0").parse().expect("Invalid input");
            if value >= 1 && value <= 20 {
                counts2[value - 1] += 1; // Increment count for the value
            }
        }

        // Compute the weighted sum
        let mut weighted_sum = 0;
        for i in 0..20 {
            let max_count = counts1[i].max(counts2[i]); // Get the maximum count for each integer
            weighted_sum += (i + 1) * max_count; // Weighted sum calculation
        }

        // Output the result for the current dataset
        println!("{}", weighted_sum);
    }
}

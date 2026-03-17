fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();

        // Read the first integer (size of the first set)
        let size_a: usize = match iter.next() {
            Some(s) => s.parse().unwrap(),
            None => continue, // If no input, skip to the next line
        };

        // Termination condition: if size_a is zero, terminate the program
        if size_a == 0 {
            break;
        }

        // Read the second integer (size of the second set)
        let size_b: usize = match iter.next() {
            Some(s) => s.parse().unwrap(),
            None => continue, // If no input, skip to the next line
        };

        // Initialize arrays to hold counts for both sets
        let mut counts_a = vec![0; 100]; // Assuming a fixed range of 0-99
        let mut counts_b = vec![0; 100]; // Assuming a fixed range of 0-99

        // Read elements for the first set
        for _ in 0..size_a {
            if let Some(num) = iter.next() {
                let index: usize = num.parse().unwrap();
                counts_a[index] += 1; // Increment count for the corresponding number
            }
        }

        // Read elements for the second set
        for _ in 0..size_b {
            if let Some(num) = iter.next() {
                let index: usize = num.parse().unwrap();
                counts_b[index] += 1; // Increment count for the corresponding number
            }
        }

        // Compute the weighted sum based on maximum counts
        let mut weighted_sum = 0;
        for i in 0..100 {
            let max_count = counts_a[i].max(counts_b[i]); // Get the maximum count
            weighted_sum += i * max_count; // Weighted sum calculation
        }

        // Output the computed weighted sum
        println!("{}", weighted_sum);
    }
}

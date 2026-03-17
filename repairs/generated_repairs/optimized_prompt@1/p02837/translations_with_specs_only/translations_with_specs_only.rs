use std::io::{self, BufRead};

fn main() {
    // Read the number of sets from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of sets
    let num_sets: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize the maximum count of set bits
    let mut max_set_bits = 0;

    // Process each set
    for _ in 0..num_sets {
        // Read the line containing the count of pairs
        let line = lines.next().unwrap().unwrap();
        let mut numbers: Vec<u32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // The first number is the count of pairs
        let count_pairs = numbers.remove(0);

        // Ensure we have the correct number of pairs
        if numbers.len() != count_pairs as usize * 2 {
            continue; // Invalid input structure, skip this set
        }

        // Evaluate all combinations of pairs
        let mut current_set_bits = 0;

        for i in 0..count_pairs {
            let a = numbers[(i * 2) as usize];
            let b = numbers[(i * 2 + 1) as usize];

            // Count the set bits in the current pair
            current_set_bits += a.count_ones() + b.count_ones();
        }

        // Update the maximum set bits found
        max_set_bits = max_set_bits.max(current_set_bits);
    }

    // Output the maximum number of set bits found
    println!("{}", max_set_bits);
}

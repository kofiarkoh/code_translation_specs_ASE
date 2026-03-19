use std::io::{self, BufRead};

fn main() {
    // Read the number of sets from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of sets
    let num_sets: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize a vector to hold the sets of pairs
    let mut sets: Vec<Vec<(i32, i32)>> = Vec::new();

    // Read each set of pairs
    for _ in 0..num_sets {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let count: usize = nums.next().unwrap() as usize; // Number of pairs in this set
        let mut pairs = Vec::new();

        for _ in 0..count {
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();
            pairs.push((a, b));
        }
        sets.push(pairs);
    }

    // Compute the maximum number of set bits from valid combinations
    let max_set_bits = compute_max_set_bits(&sets);
    
    // Output the result
    println!("{}", max_set_bits);
}

// Function to compute the maximum number of set bits from valid combinations
fn compute_max_set_bits(sets: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut max_bits = 0;

    // Iterate through all combinations of sets
    let num_sets = sets.len();
    let total_combinations = 1 << num_sets; // 2^num_sets combinations

    for combination in 0..total_combinations {
        let mut current_bits = 0;
        let mut valid_combination = true;

        for i in 0..num_sets {
            if (combination & (1 << i)) != 0 { // Check if the i-th set is included
                for &(a, b) in &sets[i] {
                    // Check validity condition (assumed to be a placeholder for actual conditions)
                    if a < 0 || b < 0 { // Example condition for validity
                        valid_combination = false;
                        break;
                    }
                    current_bits += a.count_ones() + b.count_ones(); // Count set bits
                }
            }
            if !valid_combination {
                break; // Exit early if the combination is invalid
            }
        }

        if valid_combination {
            max_bits = max_bits.max(current_bits); // Update max bits if valid
        }
    }

    max_bits as i32 // Ensure the return type is i32
}

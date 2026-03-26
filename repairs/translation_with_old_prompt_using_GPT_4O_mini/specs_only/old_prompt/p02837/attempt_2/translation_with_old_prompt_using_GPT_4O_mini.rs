use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of conditions
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse number of conditions");
    
    // Edge case: if n is 0, output 0
    if n == 0 {
        println!("0");
        return;
    }

    // Read the number of constraints for each condition
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let count: usize = line.trim().parse().expect("Failed to parse number of constraints");
        a.push(count);
    }

    // Read the constraints x and y
    let mut x = vec![vec![]; n];
    let mut y = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..a[i] {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<usize> = line.split_whitespace()
                .map(|s| s.trim().parse().expect("Failed to parse constraint"))
                .collect();
            x[i].push(parts[0]); // bit position
            y[i].push(parts[1]); // expected value
        }
    }

    let mut max_bits_set = 0;

    // Iterate over all subsets of conditions
    let total_subsets = 1 << n; // 2^n subsets
    for subset in 0..total_subsets {
        let mut valid = true;
        let mut bits_set = 0;

        // Check each condition in the current subset
        for i in 0..n {
            if (subset & (1 << i)) != 0 { // if condition i is included in the subset
                bits_set += 1; // count this condition
                // Check constraints for this condition
                for j in 0..a[i] {
                    let bit_position = x[i][j];
                    let expected_value = y[i][j];
                    // Ensure bit_position is within bounds
                    if bit_position == 0 || bit_position > 64 {
                        valid = false; // Invalid bit position
                        break;
                    }
                    let actual_value = (subset >> (bit_position - 1)) & 1; // Get the bit at position
                    if actual_value != expected_value {
                        valid = false; // This subset does not satisfy the condition
                        break;
                    }
                }
            }
            if !valid {
                break; // No need to check further if already invalid
            }
        }

        // If the subset is valid, update the maximum bits set
        if valid {
            max_bits_set = max_bits_set.max(bits_set);
        }
    }

    // Output the maximum number of bits set to 1 in any valid subset
    println!("{}", max_bits_set);
}

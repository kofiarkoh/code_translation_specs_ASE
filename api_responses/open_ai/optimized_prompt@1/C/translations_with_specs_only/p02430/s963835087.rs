use std::io::{self, Write};

fn main() {
    // Read input values for n and k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let mut values = input.split_whitespace();
    let n: usize = values.next().unwrap().parse().expect("Invalid input for n");
    let k: usize = values.next().unwrap().parse().expect("Invalid input for k");

    // Check preconditions
    if k > n {
        // If k is greater than n, no output is produced
        return;
    }

    // Generate combinations
    generate_combinations(n, k);
}

fn generate_combinations(n: usize, k: usize) {
    let mut combination = vec![0; k];
    let mut index = 0;

    // Initialize the first combination
    for i in 0..k {
        combination[i] = i;
    }

    loop {
        // Create binary pattern
        let mut pattern = 0;
        for &idx in &combination {
            pattern |= 1 << idx; // Set the bit at position idx
        }

        // Output the combination
        let indices: Vec<String> = combination.iter().map(|&i| i.to_string()).collect();
        println!("{}: {}", format!("{:0width$b}", pattern, width = n), indices.join(" "));

        // Generate the next combination
        index = k - 1;
        while index != usize::MAX {
            if combination[index] < n - 1 - (k - 1 - index) {
                combination[index] += 1;
                for j in index + 1..k {
                    combination[j] = combination[j - 1] + 1; // Increment subsequent indices
                }
                break;
            }
            index -= 1;
        }

        // If no more combinations can be generated, exit the loop
        if index == usize::MAX {
            break;
        }
    }
}

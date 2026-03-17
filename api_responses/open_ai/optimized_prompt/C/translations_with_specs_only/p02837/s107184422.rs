fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of groups
    let num_groups: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(num_groups <= 20, "Number of groups must not exceed 20");

    // Initialize a vector to hold constraints for each group
    let mut constraints: Vec<Vec<(i32, i32)>> = Vec::with_capacity(num_groups);

    // Read constraints for each group
    for _ in 0..num_groups {
        let num_constraints: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut group_constraints = Vec::with_capacity(num_constraints);
        
        for _ in 0..num_constraints {
            let pair: Vec<i32> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            group_constraints.push((pair[0], pair[1]));
        }
        
        constraints.push(group_constraints);
    }

    // Compute the maximum number of set bits
    let max_set_bits = compute_max_set_bits(&constraints);
    
    // Output the result
    println!("{}", max_set_bits);
}

// Function to compute the maximum number of set bits based on constraints
fn compute_max_set_bits(constraints: &Vec<Vec<(i32, i32)>>) -> i32 {
    let num_groups = constraints.len();
    let mut max_bits = 0;

    // Iterate through all possible combinations of groups
    for i in 0..(1 << num_groups) {
        let mut valid = true;
        let mut current_bits = 0;

        // Check each group for the current combination
        for (group_index, group_constraints) in constraints.iter().enumerate() {
            if (i & (1 << group_index)) != 0 {
                // This group is included in the combination
                current_bits += 1; // Increment bit count for this group

                // Check if the current combination satisfies all constraints
                for &(a, b) in group_constraints {
                    if !is_valid_combination(a, b) {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                break;
            }
        }

        // Update max_bits if the current combination is valid
        if valid {
            max_bits = max_bits.max(current_bits);
        }
    }

    max_bits
}

// Function to determine if a combination is valid based on constraints
fn is_valid_combination(a: i32, b: i32) -> bool {
    // Placeholder for actual validation logic based on constraints
    // This should be replaced with the actual logic as per the specification
    a <= b // Example condition, replace with actual logic
}

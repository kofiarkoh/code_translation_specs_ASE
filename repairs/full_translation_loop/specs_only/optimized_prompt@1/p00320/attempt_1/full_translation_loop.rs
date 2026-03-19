use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().take(6).collect::<Result<Vec<_>, _>>().unwrap();

    // Parse the input into pairs of integers
    let mut components = Vec::new();
    for line in input {
        let dimensions: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if dimensions.len() != 2 || dimensions.iter().any(|&d| d < 0) {
            // If input is not exactly two integers or any integer is negative, terminate with error
            eprintln!("Error: Invalid input");
            return;
        }
        components.push((dimensions[0], dimensions[1]));
    }

    // Check for valid configurations
    let result = check_valid_configuration(&components);
    
    // Output the result
    if result {
        println!("yes");
    } else {
        println!("no");
    }
}

// Function to check if a valid configuration exists
fn check_valid_configuration(components: &[(i32, i32)]) -> bool {
    // Generate all permutations of the components
    let permutations = permute(components);
    
    // Check each permutation for valid connection lengths
    for perm in permutations {
        if is_valid_configuration(&perm) {
            return true;
        }
    }
    false
}

// Function to generate permutations of the components
fn permute<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let n = items.len();
    let mut indices: Vec<usize> = (0..n).collect();
    let mut stack = vec![0];

    while let Some(i) = stack.pop() {
        if i == n {
            result.push(indices.iter().map(|&index| items[index].clone()).collect());
        } else {
            for j in i..n {
                indices.swap(i, j);
                stack.push(i + 1);
                indices.swap(i, j); // backtrack
            }
        }
    }
    result
}

// Function to check if a given permutation is a valid configuration
fn is_valid_configuration(perm: &[(i32, i32)]) -> bool {
    // Here we assume a simple check for equal connection lengths
    // This can be modified based on the actual connection criteria
    let mut lengths = HashSet::new();
    for &(h, w) in perm {
        lengths.insert(h);
        lengths.insert(w);
    }
    lengths.len() == 1 // All lengths must be equal for a valid configuration
}

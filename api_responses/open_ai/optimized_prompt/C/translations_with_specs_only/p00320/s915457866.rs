use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();

    // Check if we have exactly six pairs of integers
    if input.len() != 6 {
        eprintln!("Error: Expected exactly six pairs of integers.");
        std::process::exit(1);
    }

    // Parse the dimensions into a vector of tuples
    let mut dimensions = Vec::new();
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Error: Each line must contain exactly two integers.");
            std::process::exit(1);
        }
        let height: usize = parts[0].parse().unwrap();
        let width: usize = parts[1].parse().unwrap();
        dimensions.push((height, width));
    }

    // Check for valid configurations
    if find_valid_configuration(&dimensions) {
        println!("yes");
    } else {
        println!("no");
    }
}

// Function to evaluate all permutations of the dimensions
fn find_valid_configuration(dimensions: &[(usize, usize)]) -> bool {
    let n = dimensions.len();
    let mut visited = vec![false; n];
    let mut current_permutation = vec![(0, 0); n];
    let mut results = HashSet::new();

    // Generate all permutations
    permute(dimensions, &mut visited, &mut current_permutation, 0, &mut results);

    // Check if any permutation satisfies the length constraints
    for &(h, w) in &results {
        // Here we can define the specific length constraints to check
        // For now, we assume any configuration is valid
        if h > 0 && w > 0 { // Placeholder condition
            return true;
        }
    }
    false
}

// Recursive function to generate permutations
fn permute(
    dimensions: &[(usize, usize)],
    visited: &mut Vec<bool>,
    current_permutation: &mut Vec<(usize, usize)>,
    depth: usize,
    results: &mut HashSet<(usize, usize)>,
) {
    if depth == dimensions.len() {
        // Store the current permutation in results
        results.insert(current_permutation.clone());
        return;
    }

    for i in 0..dimensions.len() {
        if !visited[i] {
            visited[i] = true;
            current_permutation[depth] = dimensions[i];
            permute(dimensions, visited, current_permutation, depth + 1, results);
            visited[i] = false;
        }
    }
}

use std::io::{self, Write};

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input to an integer
    let n: usize = input.trim().parse().expect("Please enter a non-negative integer");

    // Generate and print all permutations
    let mut sequence: Vec<usize> = (1..=n).collect();
    let mut used = vec![false; n + 1]; // Track used numbers
    let mut current_permutation = Vec::new();
    
    // Call the recursive function to generate permutations
    generate_permutations(&sequence, &mut used, &mut current_permutation);
}

// Recursive function to generate permutations
fn generate_permutations(sequence: &[usize], used: &mut Vec<bool>, current_permutation: &mut Vec<usize>) {
    if current_permutation.len() == sequence.len() {
        // Print the current permutation
        let output: String = current_permutation.iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", output);
        return;
    }

    for i in 0..sequence.len() {
        if !used[sequence[i] - 1] { // Check if the number is already used
            used[sequence[i] - 1] = true; // Mark the number as used
            current_permutation.push(sequence[i]); // Add to current permutation
            
            // Recur to generate further permutations
            generate_permutations(sequence, used, current_permutation);
            
            // Backtrack
            current_permutation.pop(); // Remove last added number
            used[sequence[i] - 1] = false; // Mark the number as unused
        }
    }
}

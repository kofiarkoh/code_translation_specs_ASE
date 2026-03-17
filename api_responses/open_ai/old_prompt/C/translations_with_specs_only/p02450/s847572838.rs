fn main() {
    // Read the input value for n from standard input
    let n: usize = read_input(); // Assume this function reads a valid integer from input

    // Handle the case where n is 0
    if n == 0 {
        return; // No output for n = 0
    }

    // Initialize the arrays for used and result
    let mut used = vec![false; n]; // Track used integers
    let mut result = vec![0; n]; // Store the current permutation

    // Start the recursive permutation generation
    enumeration(&mut result, &mut used, n, 0);
}

// Recursive function to generate permutations
fn enumeration(result: &mut Vec<usize>, used: &mut Vec<bool>, n: usize, d: usize) {
    // Base case: if depth equals n, print the current permutation
    if d == n {
        // Print the current permutation
        println!("{}", result.iter().map(|x| (x + 1).to_string()).collect::<Vec<String>>().join(" "));
        return;
    }

    // Iterate through all possible integers from 0 to n-1
    for i in 0..n {
        if !used[i] { // If the integer is not used
            // Mark the integer as used
            used[i] = true;
            // Set the current position in result
            result[d] = i; // Store the index (0-based)

            // Recur to fill the next position
            enumeration(result, used, n, d + 1);

            // Backtrack: unmark the integer as used
            used[i] = false;
        }
    }
}

// Function to read an integer from standard input
fn read_input() -> usize {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

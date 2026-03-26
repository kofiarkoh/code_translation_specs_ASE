use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid integer");

    // Initialize arrays for used and result
    let mut used = vec![false; n];
    let mut result = vec![0; n];

    // Call the enumeration function to generate permutations
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
        // If the integer is not used, select it for the current permutation
        if !used[i] {
            used[i] = true; // Mark as used
            result[d] = i; // Store the integer in the result

            // Recur to fill the next position
            enumeration(result, used, n, d + 1);

            // Backtrack: unmark the integer and continue
            used[i] = false;
        }
    }
}

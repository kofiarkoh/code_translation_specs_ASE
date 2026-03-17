use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let n: usize = handle.lines()
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.parse().ok())
        .unwrap_or(0); // Default to 0 if parsing fails, as behavior is undefined for invalid input

    // Generate and print permutations
    let mut nums: Vec<usize> = (1..=n).collect();
    let mut visited = vec![false; n + 1]; // Track visited numbers
    let mut permutation = Vec::new();
    
    generate_permutations(&nums, &mut visited, &mut permutation);
}

// Recursive function to generate permutations
fn generate_permutations(nums: &[usize], visited: &mut Vec<bool>, permutation: &mut Vec<usize>) {
    if permutation.len() == nums.len() {
        // Print the current permutation
        println!("{}", permutation.iter().map(usize::to_string).collect::<Vec<String>>().join(" "));
        return;
    }

    for i in 0..nums.len() {
        if !visited[nums[i]] {
            visited[nums[i]] = true; // Mark as visited
            permutation.push(nums[i]); // Add to current permutation

            // Recur to generate further permutations
            generate_permutations(nums, visited, permutation);

            // Backtrack
            visited[nums[i]] = false; // Unmark as visited
            permutation.pop(); // Remove last element
        }
    }
}

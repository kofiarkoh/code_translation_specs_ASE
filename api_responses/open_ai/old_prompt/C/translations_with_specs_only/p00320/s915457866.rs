use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();

    // Check if we have exactly 6 pairs of integers
    if input.len() != 6 {
        println!("no");
        std::process::exit(1);
    }

    // Parse the dimensions of the hardware components
    let mut dimensions = Vec::new();
    for line in input {
        let parts: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if parts.len() != 2 {
            println!("no");
            std::process::exit(1);
        }
        dimensions.push(parts);
    }

    // Generate all permutations of indices [0, 1, 2, 3, 4, 5]
    let indices = [0, 1, 2, 3, 4, 5];
    let mut found_valid_configuration = false;

    // Check all permutations
    for perm in permute(&indices) {
        // Check connection constraints (length equality conditions)
        if check_constraints(&dimensions, &perm) {
            found_valid_configuration = true;
            break;
        }
    }

    // Output the result
    if found_valid_configuration {
        println!("yes");
    } else {
        println!("no");
    }
}

// Function to generate all permutations of an array
fn permute(arr: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut perm = Vec::new();
    let mut used = vec![false; arr.len()];
    backtrack(arr, &mut perm, &mut used, &mut result);
    result
}

// Backtracking function to generate permutations
fn backtrack(arr: &[usize], perm: &mut Vec<usize>, used: &mut Vec<bool>, result: &mut Vec<Vec<usize>>) {
    if perm.len() == arr.len() {
        result.push(perm.clone());
        return;
    }
    for i in 0..arr.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        perm.push(arr[i]);
        backtrack(arr, perm, used, result);
        perm.pop();
        used[i] = false;
    }
}

// Function to check if the current permutation satisfies the connection constraints
fn check_constraints(dimensions: &Vec<Vec<i32>>, perm: &[usize]) -> bool {
    // Example constraint: sum of first two dimensions must equal sum of last two
    let sum1 = dimensions[perm[0]][0] + dimensions[perm[1]][0];
    let sum2 = dimensions[perm[2]][0] + dimensions[perm[3]][0];
    let sum3 = dimensions[perm[4]][0] + dimensions[perm[5]][0];
    
    // Check if any two sums are equal (this is just an example, adjust as needed)
    sum1 == sum2 || sum1 == sum3 || sum2 == sum3
}

use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into a 2D array of dimensions
    let dimensions: Vec<(i32, i32)> = input.split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|pair| {
            (pair[0].parse().unwrap(), pair[1].parse().unwrap())
        })
        .collect();

    // Check if we have exactly 6 pairs of dimensions
    if dimensions.len() != 6 {
        println!("no");
        std::process::exit(1);
    }

    // Generate all permutations of indices 0 to 5
    let indices = [0, 1, 2, 3, 4, 5];
    let mut found_valid_configuration = false;

    // Check all permutations
    for perm in permute(&indices) {
        if check_configuration(&dimensions, &perm) {
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

// Function to generate permutations of an array
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

// Function to check if a given configuration satisfies the connection constraints
fn check_configuration(dimensions: &[(i32, i32)], perm: &[usize]) -> bool {
    // Check if the sum of the first dimensions equals the sum of the second dimensions
    let sum_first = perm.iter().map(|&i| dimensions[i].0).sum::<i32>();
    let sum_second = perm.iter().map(|&i| dimensions[i].1).sum::<i32>();
    
    // Return true if the sums are equal, indicating a valid configuration
    sum_first == sum_second
}

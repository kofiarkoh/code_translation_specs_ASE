use std::io;

fn main() {
    let n: usize = read_input(); // Read the number of elements
    let mut sequence: Vec<i32> = read_sequence(n); // Read the sequence of integers

    // Compute the previous permutation
    if let Some(prev_perm) = previous_permutation(&mut sequence.clone()) {
        println!("{:?}", prev_perm); // Print previous permutation
    }
    
    println!("{:?}", sequence); // Print original sequence

    // Compute the next permutation
    if let Some(next_perm) = next_permutation(&mut sequence.clone()) {
        println!("{:?}", next_perm); // Print next permutation
    }
}

// Function to read the number of elements
fn read_input() -> usize {
    // For simplicity, we assume valid input as per the specification
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap() // Convert to usize
}

// Function to read the sequence of integers
fn read_sequence(n: usize) -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .take(n) // Ensure we only take n elements
        .map(|s| s.parse().unwrap()) // Convert to i32
        .collect()
}

// Function to compute the previous permutation
fn previous_permutation(seq: &mut Vec<i32>) -> Option<Vec<i32>> {
    let mut i = seq.len() - 2;
    while i >= 0 && seq[i] <= seq[i + 1] {
        if i == 0 { // Prevent underflow
            return None; // No previous permutation
        }
        i -= 1;
    }

    let mut j = seq.len() - 1;
    while seq[j] >= seq[i] {
        j -= 1;
    }
    seq.swap(i, j);
    seq[i + 1..].reverse(); // Reverse the sequence after i
    Some(seq.clone()) // Return the previous permutation
}

// Function to compute the next permutation
fn next_permutation(seq: &mut Vec<i32>) -> Option<Vec<i32>> {
    let mut i = seq.len() - 2;
    while i >= 0 && seq[i] >= seq[i + 1] {
        if i == 0 { // Prevent underflow
            return None; // No next permutation
        }
        i -= 1;
    }

    let mut j = seq.len() - 1;
    while seq[j] <= seq[i] {
        j -= 1;
    }
    seq.swap(i, j);
    seq[i + 1..].reverse(); // Reverse the sequence after i
    Some(seq.clone()) // Return the next permutation
}

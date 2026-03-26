use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Ensure n is within the valid range
    assert!(n >= 1 && n <= 9, "n must be between 1 and 9");

    // Read the sequence of integers into an array
    let mut A: [i32; 9] = [0; 9];
    for i in 0..n {
        A[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Create copies for previous and next permutations
    let mut prev_A = A;
    let mut next_A = A;

    // Calculate previous permutation
    if !prev_permutation(&mut prev_A, n) {
        // If no previous permutation, it remains the same
    }

    // Calculate next permutation
    if !next_permutation(&mut next_A, n) {
        // If no next permutation, it remains the same
    }

    // Print the previous permutation
    print_permutation(&prev_A, n);
    // Print the original sequence
    print_permutation(&A, n);
    // Print the next permutation
    print_permutation(&next_A, n);
}

// Function to compute the previous permutation
fn prev_permutation(arr: &mut [i32; 9], n: usize) -> bool {
    let mut i = n - 1;
    while i > 0 && arr[i - 1] <= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false; // No previous permutation
    }

    let mut j = n - 1;
    while arr[j] >= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

// Function to compute the next permutation
fn next_permutation(arr: &mut [i32; 9], n: usize) -> bool {
    let mut i = n - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false; // No next permutation
    }

    let mut j = n - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

// Function to print the permutation
fn print_permutation(arr: &[i32; 9], n: usize) {
    for i in 0..n {
        print!("{} ", arr[i]);
    }
    println!();
}

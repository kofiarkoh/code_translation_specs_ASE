use std::io::{self, Write};

fn main() {
    // Read the number of integers
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    // Ensure n is within the valid range
    if n < 1 || n > 9 {
        panic!("n must be between 1 and 9");
    }

    // Read the sequence of integers
    let mut A = [0; 9]; // Fixed size array
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    // Copy the input into the fixed size array
    A[..n].copy_from_slice(&nums);

    // Calculate previous and next permutations
    let prev_A = previous_permutation(&A[..n]);
    let next_A = next_permutation(&A[..n]);

    // Print the results
    if let Some(prev) = prev_A {
        println!("{}", prev.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    } else {
        println!("{}", A[..n].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
    println!("{}", A[..n].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    if let Some(next) = next_A {
        println!("{}", next.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    } else {
        println!("{}", A[..n].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

// Function to find the previous permutation
fn previous_permutation(arr: &[i32]) -> Option<Vec<i32>> {
    let mut a = arr.to_vec();
    let n = a.len();
    let mut i = n - 1;

    // Find the largest index i such that a[i] > a[i - 1]
    while i > 0 && a[i - 1] <= a[i] {
        i -= 1;
    }

    // If no such index exists, this is the last permutation
    if i == 0 {
        return None;
    }

    // Find the largest index j such that a[j] < a[i - 1]
    let mut j = n - 1;
    while a[j] >= a[i - 1] {
        j -= 1;
    }

    // Swap the pivot with j
    a.swap(i - 1, j);
    // Reverse the sequence from i to end
    a[i..].reverse();
    Some(a)
}

// Function to find the next permutation
fn next_permutation(arr: &[i32]) -> Option<Vec<i32>> {
    let mut a = arr.to_vec();
    let n = a.len();
    let mut i = n - 1;

    // Find the largest index i such that a[i] < a[i + 1]
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }

    // If no such index exists, this is the last permutation
    if i == 0 {
        return None;
    }

    // Find the largest index j such that a[j] > a[i - 1]
    let mut j = n - 1;
    while a[j] <= a[i - 1] {
        j -= 1;
    }

    // Swap the pivot with j
    a.swap(i - 1, j);
    // Reverse the sequence from i to end
    a[i..].reverse();
    Some(a)
}

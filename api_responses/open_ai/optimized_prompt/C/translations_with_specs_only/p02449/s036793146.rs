fn main() {
    use std::io::{self, Write};

    // Read the number of elements in the sequence
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be an integer");

    // Check precondition for n
    if n < 1 || n > 9 {
        return; // Undefined behavior for n outside [1, 9]
    }

    // Read the sequence of integers
    let mut sequence = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    sequence.extend(input.trim().split_whitespace().filter_map(|s| s.parse::<i32>().ok()).take(n));

    // Print the previous permutation
    if let Some(prev_perm) = previous_permutation(&sequence) {
        println!("{:?}", prev_perm);
    }

    // Print the original sequence
    println!("{:?}", sequence);

    // Print the next permutation
    if let Some(next_perm) = next_permutation(&sequence) {
        println!("{:?}", next_perm);
    }
}

// Function to compute the previous permutation
fn previous_permutation(seq: &[i32]) -> Option<Vec<i32>> {
    let mut perm = seq.to_vec();
    let len = perm.len();
    
    // Find the largest index i such that perm[i] > perm[i + 1]
    let mut i = len as isize - 2;
    while i >= 0 && perm[i as usize] <= perm[(i + 1) as usize] {
        i -= 1;
    }

    // If no such index exists, this is the last permutation
    if i < 0 {
        return None;
    }

    // Find the largest index j such that perm[i] > perm[j]
    let mut j = len as isize - 1;
    while perm[i as usize] <= perm[j as usize] {
        j -= 1;
    }

    // Swap perm[i] and perm[j]
    perm.swap(i as usize, j as usize);

    // Reverse the sequence from i + 1 to the end
    perm[(i + 1) as usize..].reverse();

    Some(perm)
}

// Function to compute the next permutation
fn next_permutation(seq: &[i32]) -> Option<Vec<i32>> {
    let mut perm = seq.to_vec();
    let len = perm.len();

    // Find the largest index i such that perm[i] < perm[i + 1]
    let mut i = len as isize - 2;
    while i >= 0 && perm[i as usize] >= perm[(i + 1) as usize] {
        i -= 1;
    }

    // If no such index exists, this is the last permutation
    if i < 0 {
        return None;
    }

    // Find the largest index j such that perm[i] < perm[j]
    let mut j = len as isize - 1;
    while perm[i as usize] >= perm[j as usize] {
        j -= 1;
    }

    // Swap perm[i] and perm[j]
    perm.swap(i as usize, j as usize);

    // Reverse the sequence from i + 1 to the end
    perm[(i + 1) as usize..].reverse();

    Some(perm)
}

fn main() {
    // Read input values
    let (n, k, a) = read_input();

    // Ensure preconditions are met
    assert!(n > k, "N must be greater than K");

    // Initialize the count array with size N + 1
    let mut counts = vec![0; n + 1];

    // Count occurrences of each integer in A
    for &value in &a {
        counts[value as usize] += 1;
    }

    // Collect non-zero counts and sort them
    let mut non_zero_counts: Vec<_> = counts.iter().filter(|&&count| count > 0).collect();
    non_zero_counts.sort_unstable();

    // Calculate the sum of the first N - K unique counts
    let sum: i64 = non_zero_counts.iter()
        .take(non_zero_counts.len().saturating_sub(k)) // Ensure we don't take more than available
        .map(|&&count| count as i64)
        .sum();

    // Print the result
    println!("{}", sum);
}

// Function to read input values
fn read_input() -> (usize, usize, Vec<usize>) {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for N and K
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read the second line for the array A
    let mut second_line = String::new();
    reader.read_line(&mut second_line).unwrap();
    let a: Vec<usize> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (n, k, a)
}

use std::collections::HashSet;
use std::io::{self, BufRead};

fn count_distinct_integers(n: usize, list: Vec<i32>) -> usize {
    // Preconditions: n must be non-negative, and list must contain integers.
    // If n is zero, the output will be zero as specified.
    if n == 0 {
        return 0;
    }

    // Using a HashSet to store distinct integers.
    let mut distinct_integers = HashSet::new();

    // Insert each integer from the list into the HashSet.
    for &num in &list {
        distinct_integers.insert(num);
    }

    // The size of the HashSet represents the count of distinct integers.
    distinct_integers.len()
}

fn main() {
    // Reading input from standard input.
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of elements.
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the subsequent lines for the list of integers.
    let mut list = Vec::new();
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        list.push(num);
    }

    // Calculate and print the count of distinct integers.
    let count = count_distinct_integers(n, list);
    println!("{}", count);
}

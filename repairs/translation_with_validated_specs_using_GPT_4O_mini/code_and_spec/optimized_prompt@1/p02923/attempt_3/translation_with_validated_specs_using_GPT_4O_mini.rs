use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Make handle mutable
    
    // Read the number of elements
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Handle the case where n is zero
    if n == 0 {
        println!("0");
        return;
    }

    // Read the sequence of integers
    let mut h: Vec<i32> = Vec::with_capacity(n);
    input.clear();
    handle.read_line(&mut input).unwrap();
    h.extend(input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()));

    let mut max_length = 1; // At least one element is always a non-increasing subsequence
    let mut current_length = 1;

    for i in 1..n {
        if h[i] <= h[i - 1] {
            current_length += 1;
        } else {
            current_length = 1; // Reset count for non-increasing subsequence
        }
        max_length = max_length.max(current_length); // Update max_length if current is longer
    }

    println!("{}", max_length);
}

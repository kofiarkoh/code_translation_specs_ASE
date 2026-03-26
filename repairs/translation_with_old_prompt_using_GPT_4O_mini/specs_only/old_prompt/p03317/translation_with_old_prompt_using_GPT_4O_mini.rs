use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    
    // Parse n and k, ensuring they are valid long long int values
    let n: i64 = params.next().unwrap().parse().expect("Failed to parse n");
    let k: i64 = params.next().unwrap().parse().expect("Failed to parse k");

    // Read the array of integers
    let mut a: Vec<i64> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        // Trim whitespace to avoid parsing errors
        let value: i64 = line.trim().parse().expect("Failed to parse array value");
        a.push(value);
    }

    // Calculate the minimum number of groups
    // Since k > 1 is guaranteed, we can safely perform the division
    let groups = (n + k - 1) / k; // This emulates ceiling division for positive integers

    // Output the result
    println!("{}", groups);
}

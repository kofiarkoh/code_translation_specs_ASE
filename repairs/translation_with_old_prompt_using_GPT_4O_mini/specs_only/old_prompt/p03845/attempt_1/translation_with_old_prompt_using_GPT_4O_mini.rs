use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for n and m
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Read the array t
    let mut t = vec![0; n];
    let mut t_line = String::new();
    reader.read_line(&mut t_line).unwrap();
    for (i, value) in t_line.split_whitespace().enumerate() {
        t[i] = value.parse().unwrap();
    }

    // Process each update
    for _ in 0..m {
        let mut update_line = String::new();
        reader.read_line(&mut update_line).unwrap();
        let mut update_parts = update_line.split_whitespace();
        let p: usize = update_parts.next().unwrap().parse().unwrap();
        let x: i32 = update_parts.next().unwrap().parse().unwrap();

        // Update the specified element in t
        let index = p - 1; // Convert to zero-based index
        t[index] = x;

        // Calculate the modified sum
        let modified_sum: i32 = t.iter().sum();
        
        // Output the modified sum
        println!("{}", modified_sum);
    }
}

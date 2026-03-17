use std::io;

fn compute_value(n: u64, k: u64) -> u64 {
    // Base case: if n is 0, return 0 (as the sum of k added n times)
    if n == 0 {
        return 0;
    }
    // Recursive case: compute the value based on n and k
    compute_value(n - 1, k) + k
}

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as u64
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let n: u64 = parts[0].parse().expect("Invalid input for n");
    let k: u64 = parts[1].parse().expect("Invalid input for k");
    
    // Compute the value based on the inputs
    let result = compute_value(n, k);
    
    // Output the result
    println!("{}", result);
}

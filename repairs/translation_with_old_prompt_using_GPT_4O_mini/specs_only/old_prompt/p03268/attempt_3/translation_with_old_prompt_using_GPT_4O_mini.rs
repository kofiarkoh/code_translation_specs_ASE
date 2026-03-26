use std::io;

fn main() {
    // Read input values for N and K
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse N and K as u64 to handle large values
    let n: u64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: u64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Check if K is zero to avoid division by zero
    if k == 0 {
        // Undefined behavior as per specification; we can choose to panic here
        panic!("Division by zero: K should not be zero");
    }

    // Calculate the number of complete groups of size K
    let complete_groups = n / k;

    // Compute the result based on the parity of K
    let result = if k % 2 == 0 {
        // If K is even, adjust the result by adding 1 before cubing
        (complete_groups + 1).pow(3)
    } else {
        // If K is odd, simply cube the complete groups
        complete_groups.pow(3)
    };

    // Output the result
    println!("{}", result);
}

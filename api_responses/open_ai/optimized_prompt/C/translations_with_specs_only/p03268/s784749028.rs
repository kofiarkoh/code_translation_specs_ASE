use std::io;

fn main() {
    // Read input values for N and K
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.split_whitespace();
    
    // Parse N and K as i64 (equivalent to long long in C)
    let n: i64 = numbers.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = numbers.next().unwrap().parse().expect("Invalid input for K");

    // Check for division by zero (K must not be zero)
    if k == 0 {
        // Division by zero leads to undefined behavior; we do nothing here as per spec
        return;
    }

    // Calculate the quotient of N divided by K
    let quotient = n / k;

    // Calculate the cube of the quotient
    let result = quotient.pow(3);

    // If K is odd, we need to add another cube based on a modified value of N
    let final_result = if k % 2 != 0 {
        // Calculate the cube of (N - K)
        let modified_n = n - k;
        result + modified_n.pow(3)
    } else {
        result
    };

    // Print the result, ensuring it is non-negative
    println!("{}", final_result);
}

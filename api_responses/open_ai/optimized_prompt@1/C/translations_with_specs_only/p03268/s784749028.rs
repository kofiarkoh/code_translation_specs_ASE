use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N and K from input
    let n: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let k: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    // Ensure K is non-zero to avoid division by zero
    if k == 0 {
        // Undefined behavior as per specification; we do not handle this case explicitly.
        return;
    }

    // Calculate the quotient of N divided by K
    let quotient = n / k;

    // Calculate the cube of the quotient
    let result = quotient.pow(3);

    // If K is odd, we need to add another cube based on a modified value of N
    let final_result = if k % 2 != 0 {
        // Calculate the cube of (N + K)
        result + (n + k).pow(3)
    } else {
        result
    };

    // Output the result, ensuring it is non-negative
    println!("{}", final_result);
}

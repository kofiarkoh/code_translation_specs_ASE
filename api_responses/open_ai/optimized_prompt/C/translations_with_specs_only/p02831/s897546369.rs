use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first input (non-zero integer)
    let first_input: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the second input (positive integer)
    let second_input: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions check
    // The first input must be a non-zero integer
    // The second input must be a positive integer
    if first_input == 0 || second_input <= 0 {
        // Behavior is not defined for invalid inputs, so we do nothing
        return;
    }

    // Compute the smallest multiple of `first_input` that is also a multiple of `second_input`
    let mut multiple = first_input;

    // We will iterate to find the smallest valid multiple
    while multiple % second_input != 0 {
        multiple += first_input; // Increment by `first_input` to find the next multiple
    }

    // Output the result
    // The output is guaranteed to be a long integer that is a multiple of both inputs
    println!("{}", multiple);
}

use std::io::{self, BufRead};

const MODULUS: i32 = 998244353;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut inputs: Vec<i32> = Vec::new();

    // Read inputs from standard input
    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let number: i32 = line.trim().parse().expect("Input must be an integer");
        inputs.push(number);
    }

    // Ensure the first input is non-negative
    let first_input = inputs.get(0).expect("No input provided");
    assert!(*first_input >= 0, "The first input must be a non-negative integer");

    // Initialize arrays for accumulation
    let mut array1 = vec![0; 2]; // Assuming size 2 based on the specification
    let mut array2 = vec![0; 2]; // Assuming size 2 based on the specification

    // Process each input and update accumulations
    for &input in &inputs {
        // Update the first array with the input
        array1[0] = (array1[0] + input) % MODULUS;
        // Update the second array with the input
        array2[0] = (array2[0] + input) % MODULUS;
    }

    // Compute the final result based on the accumulations
    let result = (array1[0] + array2[0]) % MODULUS;

    // Output the result
    println!("{}", result);
}

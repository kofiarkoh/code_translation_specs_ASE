use std::io::{self, BufRead};

fn main() {
    // Read all input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the count of subsequent integers
    let count: usize = lines.next()
        .expect("Expected a count of integers")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Expected a non-negative integer");

    // Initialize arrays and result
    let mut array1 = vec![0; count];
    let mut array2 = vec![0; count];
    let modulus: i32 = 998244352;
    let mut result: i32 = 0;

    // Process the subsequent integers
    for (i, line) in lines.take(count).enumerate() {
        let value: i32 = line
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Expected an integer");

        // Update arrays and result based on the input value
        array1[i] = (array1[i] + value) % modulus; // Maintain modular constraint
        array2[i] = (array2[i] + value) % modulus; // Maintain modular constraint
        result = (result + array1[i] + array2[i]) % modulus; // Update result
    }

    // Output the final result
    println!("{}", result);
}

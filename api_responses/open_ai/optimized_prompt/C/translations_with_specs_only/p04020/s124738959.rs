fn main() {
    use std::io::{self, BufRead};

    // Read the number of elements in the sequence
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Ensure n is positive as per the specification
    assert!(n > 0, "The number of elements must be positive.");

    // Read the sequence of integers
    let mut sequence: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        sequence.push(value);
    }

    // Compute the result based on the specified rules
    let mut result: i64 = 0;
    for i in 0..n {
        let half_value = sequence[i] / 2; // Integer division to get half
        result += half_value;

        // Adjust based on the parity of adjacent integers
        if i > 0 {
            if (sequence[i] % 2) != (sequence[i - 1] % 2) {
                result += 1; // Adjust if current and previous have different parity
            }
        }
    }

    // Output the computed value
    println!("{}", result);
}

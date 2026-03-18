use std::io::{self, BufRead};

fn main() {
    // Read the number of integers
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Ensure N is within the specified bounds
    assert!(n >= 1 && n <= 100_000);

    let mut sum: u64 = 0; // Using u64 to ensure non-negative output

    // Vector to store the integers
    let mut integers = Vec::with_capacity(n);

    // Read the integers and compute the sum
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        integers.push(value);
    }

    // Calculate the sum based on the specification
    for i in 0..n {
        let half = (integers[i] / 2) as u64; // Compute half of the integer
        sum += half; // Add half to the sum

        // Adjust based on the parity of adjacent integers
        if i > 0 {
            if (integers[i] % 2) != (integers[i - 1] % 2) {
                sum += 1; // Adjust if current and previous integers have different parity
            }
        }
    }

    // Output the computed value
    println!("{}", sum);
}

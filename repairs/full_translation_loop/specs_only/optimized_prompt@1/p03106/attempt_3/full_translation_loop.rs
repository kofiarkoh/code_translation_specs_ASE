use std::io::{self, BufRead};

fn gcd(a: u32, b: u32) -> u32 {
    // Calculate the GCD using the Euclidean algorithm
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn count_common_divisors(n: u32) -> u32 {
    // Count the number of divisors of n
    let mut count = 0;
    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line and parse the integers
    if let Some(Ok(line)) = lines.next() {
        let nums: Vec<u32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() == 3 {
            let a = nums[0]; // First integer
            let b = nums[1]; // Second integer
            let expected_count = nums[2]; // Expected count of common divisors

            // Calculate GCD
            let gcd_value = gcd(a, b);
            
            // Count common divisors of the GCD
            let common_divisor_count = count_common_divisors(gcd_value);
            
            // Output the GCD if the count matches the expected value
            if common_divisor_count == expected_count {
                println!("{}", gcd_value);
            } else {
                // If the count does not match, we do not output anything (undefined behavior)
                return;
            }
        }
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let (a, b, c) = (nums[0], nums[1], nums[2]);

    // Preconditions: a, b, and c must be non-negative
    if a < 0 || b < 0 || c < 0 {
        // Undefined behavior as per specification
        return;
    }

    let mut sum = 0;
    let mut gcd = 0;

    // Determine the range for finding common divisors
    let max_divisor = if a > b { a } else { b };

    for i in (1..=max_divisor).rev() {
        if a % i == 0 && b % i == 0 {
            sum += 1;
            if sum == c {
                gcd = i;
                break;
            }
        }
    }

    // Output the GCD if the count of common divisors matches the specified value
    if sum == c {
        println!("{}", gcd);
    }
}

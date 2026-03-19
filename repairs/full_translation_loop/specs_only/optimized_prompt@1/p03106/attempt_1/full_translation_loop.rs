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
    // Example input values (to be replaced with actual input handling)
    let a: u32 = 12; // First integer
    let b: u32 = 8;  // Second integer
    let expected_count: u32 = 4; // Expected count of common divisors

    // Calculate GCD
    let gcd_value = gcd(a, b);
    
    // Count common divisors of the GCD
    let common_divisor_count = count_common_divisors(gcd_value);
    
    // Output the GCD if the count matches the expected value
    if common_divisor_count == expected_count {
        println!("{}", gcd_value);
    }
    // If the count does not match, no output is produced (undefined behavior)
}

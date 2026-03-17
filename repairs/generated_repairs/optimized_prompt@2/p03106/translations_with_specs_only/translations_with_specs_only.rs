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

fn count_common_divisors(gcd: u32) -> u32 {
    // Count the number of divisors of the GCD
    let mut count = 0;
    for i in 1..=gcd {
        if gcd % i == 0 {
            count += 1;
        }
    }
    count
}

fn calculate_gcd_and_count(a: u32, b: u32, expected_count: u32) -> Option<u32> {
    // Calculate the GCD of a and b
    let gcd_value = gcd(a, b);
    
    // Count the common divisors of the GCD
    let common_divisor_count = count_common_divisors(gcd_value);
    
    // Return the GCD if the count matches the expected count
    if common_divisor_count == expected_count {
        Some(gcd_value)
    } else {
        None // Undefined behavior if the count does not match
    }
}

fn main() {
    // Read input values
    let a = 12; // Example input
    let b = 18; // Example input
    let expected_count = 3; // Expected number of common divisors for GCD(12, 18)

    // Call the function and handle the output
    match calculate_gcd_and_count(a, b, expected_count) {
        Some(gcd) => println!("{}", gcd),
        None => println!("{}", count_common_divisors(gcd(a, b))), // Output the count of divisors if GCD does not match
    }
}

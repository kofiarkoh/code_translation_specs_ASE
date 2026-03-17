fn gcd(a: u32, b: u32) -> u32 {
    // Compute the greatest common divisor using the Euclidean algorithm
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn common_divisors(a: u32, b: u32) -> Vec<u32> {
    let mut divisors = Vec::new();
    let limit = gcd(a, b); // Get the GCD to limit the search for common divisors

    // Find all common divisors of a and b
    for i in 1..=limit {
        if a % i == 0 && b % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn largest_common_divisor_at_position(a: u32, b: u32, position: usize) -> Option<u32> {
    // Preconditions: a and b must be non-negative, position must be positive
    if position == 0 {
        return None; // Position must be a positive integer
    }

    let divisors = common_divisors(a, b);
    
    // Check if the position exceeds the number of common divisors
    if position > divisors.len() {
        return None; // Undefined behavior as per specification
    }

    // Return the largest common divisor at the specified position
    // Position is 1-based, so we access position - 1
    Some(divisors[divisors.len() - position])
}

fn main() {
    let a = 12; // Example input
    let b = 18; // Example input
    let position = 2; // Example position

    match largest_common_divisor_at_position(a, b, position) {
        Some(divisor) => println!("The largest common divisor at position {} is {}", position, divisor),
        None => println!("Undefined value: position exceeds the number of common divisors"),
    }
}

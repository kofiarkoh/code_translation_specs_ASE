fn smallest_multiple(first: i32, second: i32) -> Option<i32> {
    // Preconditions: first must be non-zero, second must be positive
    if first == 0 || second <= 0 {
        // Undefined behavior as per specification; no output guaranteed
        return None;
    }

    // Start iterating over multiples of `first`
    let mut multiple = first;

    // We will iterate until we find a valid multiple or exceed a reasonable limit
    // Since we are looking for the smallest multiple, we can stop when we reach
    // a multiple that is larger than the product of first and second.
    let limit = first * second;

    while multiple <= limit {
        if multiple % second == 0 {
            return Some(multiple); // Found the smallest multiple
        }
        multiple += first; // Move to the next multiple
    }

    // If we exit the loop without finding a valid output, return None
    None
}

// Example usage
fn main() {
    let first = 12;
    let second = 15;
    match smallest_multiple(first, second) {
        Some(result) => println!("{}", result),
        None => println!("No valid multiple found."),
    }
}

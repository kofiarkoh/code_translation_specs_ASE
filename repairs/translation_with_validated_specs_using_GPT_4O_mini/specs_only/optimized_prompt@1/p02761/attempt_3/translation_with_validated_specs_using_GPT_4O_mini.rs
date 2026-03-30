fn compute_number(n: usize, m: usize, pairs: &[(usize, usize)]) -> i32 {
    // Preconditions
    if n == 0 || m > n {
        return -1; // Invalid input based on the specification
    }

    // Initialize the digit array with -1 (unassigned)
    let mut digits = vec![-1; n];

    // Process the digit-value pairs
    for &(position, value) in pairs {
        // Check if the position is valid
        if position < 1 || position > n || value > 9 {
            return -1; // Invalid position or value
        }

        // Convert 1-based index to 0-based
        let index = position - 1;

        // Check for conflicting values
        if digits[index] != -1 && digits[index] != value as i32 {
            return -1; // Conflict detected
        }

        // Assign the value to the digit position
        digits[index] = value as i32;
    }

    // Handle special cases for M = 0
    if m == 0 {
        match n {
            1 => return 0,
            2 => return 10,
            3 => return 100,
            _ => return -1, // For N > 3, unspecified behavior
        }
    }

    // Check the first digit position
    if digits[0] == 0 && n > 1 {
        return -1; // Invalid first digit
    }

    // Compute the final number
    let mut result = 0;
    for (i, &digit) in digits.iter().enumerate() {
        if digit == -1 {
            // Unassigned positions contribute 0
            continue;
        }
        result += digit * 10_i32.pow((n - 1 - i) as u32);
    }

    // Ensure that unassigned positions are treated as zeros
    if result == 0 && n > 1 && digits[0] == -1 {
        return -1; // Invalid case where the first digit is unassigned in a multi-digit number
    }

    result
}

fn main() {
    // Example usage
    let n = 3;
    let m = 2;
    let pairs = [(1, 1), (3, 5)];
    let result = compute_number(n, m, &pairs);
    println!("{}", result); // Expected output: 105
}

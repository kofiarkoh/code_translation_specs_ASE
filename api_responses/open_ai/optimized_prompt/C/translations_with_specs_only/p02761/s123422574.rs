fn compute_value(n: usize, m: usize, pairs: &[(usize, i32)]) -> String {
    // Preconditions
    if n == 0 || m > n {
        return "-1".to_string(); // Invalid input case
    }

    // Initialize the digit array with -1
    let mut digits = vec![-1; n];

    // Handle the digit-value pairs
    for &(position, value) in pairs.iter() {
        // Check if the position is valid (1 to N)
        if position < 1 || position > n {
            return "-1".to_string(); // Invalid position
        }

        let index = position - 1; // Convert to 0-based index

        // Check for conflicts in value assignment
        if digits[index] == -1 {
            digits[index] = value; // Assign value if not already assigned
        } else if digits[index] != value {
            return "-1".to_string(); // Conflict detected
        }
    }

    // Special case handling
    if m == 0 {
        if n > 3 {
            return "-1".to_string(); // Undefined output
        } else {
            return "0".to_string(); // If M is zero and N <= 3, return 0
        }
    }

    // Check if the first digit is zero when N > 1
    if n > 1 && digits[0] == 0 {
        return "-1".to_string(); // Invalid case
    }

    // Compute the final value
    let mut result = 0;
    for (i, &value) in digits.iter().enumerate() {
        if value != -1 {
            result += value * 10_i32.pow((n - i - 1) as u32); // Construct the number
        }
    }

    result.to_string() // Return the computed value as a string
}

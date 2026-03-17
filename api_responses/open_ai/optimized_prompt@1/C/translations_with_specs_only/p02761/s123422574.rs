fn compute_value(n: usize, m: usize, pairs: &[(usize, i32)]) -> String {
    // Preconditions
    if n == 0 || m > n {
        return "-1".to_string(); // Invalid input case
    }

    // Initialize the digit array with -1
    let mut digits = vec![-1; n];

    // Handle the case where M is zero
    if m == 0 {
        if n > 3 {
            return String::new(); // No output generated
        }
        return "0".to_string(); // Default output for M = 0 and N <= 3
    }

    // Process the digit-position pairs
    for &(pos, digit) in pairs {
        // Validate position
        if pos < 1 || pos > n {
            return "-1".to_string(); // Invalid position
        }

        let index = pos - 1; // Convert to zero-based index

        // Check for inconsistency
        if digits[index] != -1 && digits[index] != digit {
            return "-1".to_string(); // Inconsistent assignment
        }

        // Assign the digit value
        digits[index] = digit;
    }

    // Special case: if the first position is 1 with a digit value of 0
    if digits[0] == 0 {
        return "-1".to_string(); // Output -1 for this specific case
    }

    // Compute the sum of the digit values
    let sum: i32 = digits.iter().sum();

    // Return the computed value as a string
    sum.to_string()
}

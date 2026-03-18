fn compute_value(n: usize, m: usize, pairs: &[(usize, i32)]) -> String {
    // Preconditions check
    if n == 0 || m > n {
        return String::from("-1");
    }

    // Initialize the digit array with -1
    let mut digits = vec![-1; n];

    // Handle the pairs and check for inconsistencies
    for &(pos, value) in pairs {
        // Check if position is valid
        if pos < 1 || pos > n {
            return String::from("-1");
        }
        let index = pos - 1; // Convert to 0-based index

        // Check for inconsistency
        if digits[index] != -1 && digits[index] != value {
            return String::from("-1");
        }
        digits[index] = value;
    }

    // Special case: if M is zero and N > 3, no output is generated
    if m == 0 && n > 3 {
        return String::new(); // No output
    }

    // Special case: if the first position is specified as 1 with a digit value of 0
    if pairs.get(0).map_or(false, |&(pos, value)| pos == 1 && value == 0) {
        return String::from("-1");
    }

    // Compute the sum of the digit values
    let sum: i32 = digits.iter().filter(|&&d| d != -1).sum();

    // Return the computed value as a string
    sum.to_string()
}

fn main() {
    // Example usage
    let n = 5;
    let m = 3;
    let pairs = [(1, 2), (2, 3), (3, 4)];
    let result = compute_value(n, m, &pairs);
    println!("{}", result);
}

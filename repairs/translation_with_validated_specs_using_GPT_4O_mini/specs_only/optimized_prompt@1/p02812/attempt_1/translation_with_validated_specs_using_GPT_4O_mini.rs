fn count_abc_occurrences(n: usize, input: &str) -> usize {
    // Preconditions check
    if n < 3 || input.len() < n || input.len() > 50 {
        // Undefined behavior as per specification; returning 0 for safety
        return 0;
    }

    let mut count = 0;
    let target = "ABC";
    let target_len = target.len();

    // Iterate through the string while ensuring we do not exceed the bounds defined by n
    for i in 0..=n - target_len {
        // Check for the substring "ABC"
        if &input[i..i + target_len] == target {
            count += 1; // Increment count for each occurrence found
        }
    }

    count // Return the total count of occurrences
}

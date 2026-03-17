fn max_length_of_ones(n: usize, k: usize, binary_string: &str) -> usize {
    // Preconditions check
    if n == 0 {
        // If the first integer input is zero, return zero as specified
        return 0;
    }
    
    // The length of the binary string must match the first integer input
    if binary_string.len() != n {
        // Behavior is not defined if the string is shorter than expected
        panic!("Input string length does not match the specified length.");
    }

    let mut left = 0; // Left pointer for the sliding window
    let mut zero_count = 0; // Count of '0's in the current window
    let mut max_length = 0; // Maximum length of contiguous '1's found

    // Iterate over the binary string with a right pointer
    for right in 0..n {
        // If we encounter a '0', increment the zero count
        if binary_string.chars().nth(right) == Some('0') {
            zero_count += 1;
        }

        // If the zero count exceeds the allowed changes, move the left pointer
        while zero_count > k {
            if binary_string.chars().nth(left) == Some('0') {
                zero_count -= 1; // Decrease zero count if we move past a '0'
            }
            left += 1; // Move the left pointer to shrink the window
        }

        // Calculate the maximum length of contiguous '1's
        max_length = max_length.max(right - left + 1);
    }

    max_length // Return the maximum length found
}

fn main() {
    // Example usage
    let n = 5; // Number of binary values
    let k = 1; // Number of changes allowed
    let binary_string = "10011"; // Input binary string

    let result = max_length_of_ones(n, k, binary_string);
    println!("{}", result); // Output the result
}

fn longest_non_increasing_subsequence(n: usize, sequence: Vec<i32>) -> usize {
    // If n is zero, return zero as there are no elements to process.
    if n == 0 {
        return 0;
    }

    let mut max_length = 1; // Maximum length of non-increasing subsequence found
    let mut current_length = 1; // Current length of the non-increasing subsequence

    // Iterate through the sequence starting from the second element
    for i in 1..n {
        // Check if the current element is less than or equal to the previous one
        if sequence[i] <= sequence[i - 1] {
            current_length += 1; // Increment current length
        } else {
            // Update max_length if current_length is greater
            if current_length > max_length {
                max_length = current_length;
            }
            current_length = 1; // Reset current length
        }
    }

    // Final check to update max_length after the loop
    if current_length > max_length {
        max_length = current_length;
    }

    max_length // Return the maximum length found
}

fn main() {
    // Example usage
    let n = 5;
    let sequence = vec![5, 3, 3, 2, 1];
    let result = longest_non_increasing_subsequence(n, sequence);
    println!("{}", result); // Output: 5
}

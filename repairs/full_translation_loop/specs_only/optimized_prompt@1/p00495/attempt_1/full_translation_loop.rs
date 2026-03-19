fn longest_subsequence_length(seq1: &[i32], seq2: &[i32]) -> i32 {
    // Preconditions: lengths of sequences must be non-negative
    let len1 = seq1.len() as i32;
    let len2 = seq2.len() as i32;

    // Invariants: sequences remain unchanged, and max_length is updated only if a longer subsequence is found
    let mut max_length = 0;

    // Iterate through each starting position in seq2
    for start in 0..len2 {
        let mut current_length = 0;

        // Check for subsequence starting from 'start' in seq2
        for &element in seq2.iter().skip(start as usize) {
            // Check if the element exists in seq1
            if seq1.contains(&element) {
                current_length += 1; // Increment current subsequence length
            }
        }

        // Update max_length if a longer subsequence is found
        if current_length > max_length {
            max_length = current_length;
        }
    }

    // Postcondition: output is non-negative and represents the length of the longest valid subsequence
    max_length
}

// Example usage (not part of the specification):
// fn main() {
//     let seq1 = vec![1, 2, 3];
//     let seq2 = vec![2, 1, 3, 4];
//     let result = longest_subsequence_length(&seq1, &seq2);
//     println!("Longest subsequence length: {}", result); // Expected output: 3
// }

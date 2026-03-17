fn longest_increasing_subsequence_length(sequence: Vec<i32>) -> usize {
    // The first element indicates the number of subsequent integers.
    let n = sequence[0] as usize;

    // We will use a vector to store the longest increasing subsequence.
    let mut lis: Vec<i32> = Vec::new();

    // Iterate through the input sequence starting from the second element.
    for &num in &sequence[1..=n] {
        // Use binary search to find the insertion point for the current number.
        let pos = lis.binary_search(&num).unwrap_or_else(|e| e);

        // If pos is equal to the length of lis, it means num is greater than
        // all elements in lis, and we can extend the subsequence.
        if pos == lis.len() {
            lis.push(num);
        } else {
            // Otherwise, we replace the existing value to maintain the property
            // of the longest increasing subsequence.
            lis[pos] = num;
        }
    }

    // The length of the longest increasing subsequence is the size of lis.
    lis.len()
}

fn main() {
    // Example usage:
    let input = vec![5, 10, 20, 10, 30, 20]; // First element is the count
    let length = longest_increasing_subsequence_length(input);
    println!("Length of longest increasing subsequence: {}", length);
}

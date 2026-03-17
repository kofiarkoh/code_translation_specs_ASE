fn can_transform_to_non_increasing(n: usize, sequence: Vec<i32>) -> &'static str {
    // Check if the sequence can be transformed into a non-increasing order
    // by decrementing elements.
    
    // We start from the second last element and move towards the first element.
    for i in (1..n).rev() {
        // If the current element is greater than the previous one,
        // we can decrement the current element to match the previous one.
        if sequence[i] > sequence[i - 1] {
            // Decrement the current element to the value of the previous one
            // to maintain the non-increasing property.
            // This simulates the decrementing process.
            // However, we do not actually modify the original sequence.
            // We just check if it's possible.
            // If we can't decrement enough, we return "No".
            if sequence[i] - sequence[i - 1] > sequence[i] {
                return "No";
            }
        }
    }
    // If we can adjust all elements to be non-increasing, we return "Yes".
    "Yes"
}

fn main() {
    // Example usage:
    let n = 5; // Number of elements
    let sequence = vec![5, 3, 4, 2, 1]; // Input sequence

    // Call the function and print the result
    let result = can_transform_to_non_increasing(n, sequence);
    println!("{}", result);
}

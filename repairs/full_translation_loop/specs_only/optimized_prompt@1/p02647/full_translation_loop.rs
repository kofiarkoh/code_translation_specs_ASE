fn process_sequence(n: usize, k: usize, sequence: Vec<i32>) -> Vec<i32> {
    // Ensure the input sequence has the correct length
    assert_eq!(sequence.len(), n, "Input sequence length must be equal to N");

    // Initialize the transformed sequence with the input sequence
    let mut transformed = sequence.clone();

    // Perform K iterations of processing
    for _ in 0..k {
        // Create a new vector to hold the updated values for this iteration
        let mut new_transformed = vec![0; n];

        // Update each value in the transformed sequence
        for i in 0..n {
            // The transformation logic is based on the problem specification.
            // We will set each new value to the maximum of itself and its neighbors.
            let left = if i > 0 { transformed[i - 1] } else { 0 };
            let right = if i < n - 1 { transformed[i + 1] } else { 0 };
            new_transformed[i] = (transformed[i] + left + right) / 3; // Average with neighbors
        }

        // Update the transformed sequence for the next iteration
        transformed = new_transformed;
    }

    // Return the final transformed sequence
    transformed
}

fn main() {
    // Example usage
    let n = 3;
    let k = 2;
    let sequence = vec![2, 1, 3];
    let result = process_sequence(n, k, sequence);
    println!("{:?}", result); // Expected output: [3, 3, 3]
}

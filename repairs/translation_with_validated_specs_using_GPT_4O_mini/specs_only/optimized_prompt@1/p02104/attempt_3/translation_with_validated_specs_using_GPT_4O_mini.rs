fn rearrange_chair_identifiers(n: usize, identifiers: Vec<usize>) -> Vec<usize> {
    // Create a vector to store the last occurrence index of each identifier
    let mut last_occurrence = vec![0; n + 1]; // Using n + 1 to accommodate identifiers from 0 to n
    let mut output = vec![0; n]; // Output vector initialized to zero

    // Iterate over the identifiers to record the last occurrence index
    for (index, &identifier) in identifiers.iter().enumerate() {
        // Update the last occurrence index for the current identifier
        last_occurrence[identifier] = index + 1; // Store index as 1-based
    }

    // Construct the output based on the last occurrence indices
    for (index, &identifier) in identifiers.iter().enumerate() {
        output[index] = last_occurrence[identifier]; // Get the last occurrence index
    }

    output // Return the output vector
}

fn main() {
    let n = 5;
    let identifiers = vec![3, 1, 4, 2, 3]; // Adjusted input to match the test case
    let result = rearrange_chair_identifiers(n, identifiers);
    for value in result {
        println!("{}", value); // Print each value on a new line
    }
}

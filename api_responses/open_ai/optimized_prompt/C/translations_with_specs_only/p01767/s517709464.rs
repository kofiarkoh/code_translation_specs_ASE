fn main() {
    // Read the size of the first array
    let size1: usize = read_input(); // Assume read_input() reads an integer from input
    let array1: Vec<i32> = read_array(size1); // Assume read_array(size) reads an array of integers

    // Read the size of the second array
    let size2: usize = read_input(); // Assume read_input() reads an integer from input
    let array2: Vec<i32> = read_array(size2); // Assume read_array(size) reads an array of integers

    // Read the long long integer array
    let long_long_array: Vec<i64> = read_array(size2); // Assume read_array(size) reads an array of long long integers

    // Preconditions check
    assert!(size1 > 0, "First array must contain at least one element.");
    assert!(size2 > 0, "Second array must contain at least one element.");
    assert!(array2.len() == long_long_array.len(), "Second array and long long array must be of equal size.");

    // Compute cumulative sums
    let mut cumulative_sum = vec![0; size1];
    cumulative_sum[0] = array1[0] as i64; // Cast to i64 for cumulative sum

    for i in 1..size1 {
        cumulative_sum[i] = cumulative_sum[i - 1] + array1[i] as i64; // Cumulative sum
    }

    // Prepare output
    let mut output = Vec::new();

    // Check conditions for each element in the second array
    for &value in &array2 {
        let index = value as usize; // Assuming value is a valid index
        if index < cumulative_sum.len() {
            // Compare cumulative sum with corresponding long long value
            if cumulative_sum[index] >= long_long_array[index] {
                output.push("Yes".to_string());
            } else {
                output.push("No".to_string());
            }
        } else {
            // If index is out of bounds, behavior is not defined; handle gracefully
            output.push("No".to_string()); // Default response for out-of-bounds
        }
    }

    // Output results
    for response in output {
        println!("{}", response);
    }
}

// Placeholder functions for reading input
fn read_input() -> usize {
    // Implementation for reading a single integer from input
    unimplemented!()
}

fn read_array<T: std::str::FromStr>(size: usize) -> Vec<T> {
    // Implementation for reading an array of specified type from input
    unimplemented!()
}

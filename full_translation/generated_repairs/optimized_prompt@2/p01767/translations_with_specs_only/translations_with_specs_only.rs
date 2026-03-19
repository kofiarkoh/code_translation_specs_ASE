fn main() {
    // Read the number of elements in the first array
    let n: usize = read_input(); // Assume read_input() reads an integer from input
    // Read the first array of integers
    let mut first_array: Vec<i32> = read_input_array(n); // Assume read_input_array() reads an array of integers
    // Read the number of elements in the second array
    let m: usize = read_input(); // Assume read_input() reads an integer from input
    // Read the second array of integers
    let second_array: Vec<i32> = read_input_array(m); // Assume read_input_array() reads an array of integers
    // Read the corresponding long long integers
    let long_long_array: Vec<i64> = read_input_array(m); // Assume read_input_array() reads an array of long long integers

    // Preconditions check
    if n == 0 || m == 0 || long_long_array.len() != m {
        return; // Behavior is not defined for invalid input
    }

    // Sort the first array to maintain the invariant
    first_array.sort();

    // Compute cumulative sums
    let mut cumulative_sums: Vec<i64> = vec![0; n];
    cumulative_sums[0] = first_array[0] as i64; // Cast to long long (i64)
    for i in 1..n {
        cumulative_sums[i] = cumulative_sums[i - 1] + first_array[i] as i64; // Cumulative sum
    }

    // Output results based on the second array and long long integers
    for &index in &second_array {
        let upper_bound_index = index as usize; // Cast to usize for indexing
        if upper_bound_index < n {
            // Check if cumulative sum at the upper bound index meets or exceeds the corresponding long long integer
            if cumulative_sums[upper_bound_index] >= long_long_array[upper_bound_index] {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            // If index is out of bounds, output "No" (as a safe fallback)
            println!("No");
        }
    }
}

// Placeholder function to read an integer from input
fn read_input() -> usize {
    // Implementation to read an integer from input
    unimplemented!()
}

// Placeholder function to read an array of integers from input
fn read_input_array<T: std::str::FromStr>(size: usize) -> Vec<T> {
    // Implementation to read an array of integers from input
    unimplemented!()
}

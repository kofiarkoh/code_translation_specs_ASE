fn main() {
    // Read N and K from input
    let (n, k): (usize, usize) = read_input(); // Assume read_input is defined elsewhere

    // Read the array of N integers
    let arr: Vec<i32> = read_array(n); // Assume read_array is defined elsewhere

    // Create a counting array to count occurrences of each integer
    let mut counts = vec![0; 100]; // Assuming the valid range of integers is 0-99

    // Count occurrences of each integer
    for &num in &arr {
        if num >= 0 && num < 100 { // Check bounds as per specification
            counts[num as usize] += 1;
        }
    }

    // Collect unique counts and filter out zeros
    let mut unique_counts: Vec<i32> = counts.iter().cloned().filter(|&count| count > 0).collect();

    // Sort the unique counts in non-decreasing order
    unique_counts.sort_unstable(); // Using sort_unstable for better performance

    // Calculate the sum of the first N - K unique counts
    let sum: i32 = if unique_counts.len() >= n - k {
        unique_counts.iter().take(n - k).sum()
    } else {
        unique_counts.iter().sum() // Return the sum of all unique counts if fewer than N - K
    };

    // Output the result
    println!("{}", sum);
}

// Placeholder function to read input values
fn read_input() -> (usize, usize) {
    // Implementation to read N and K from input
    // Example implementation (replace with actual input reading logic)
    (5, 2) // Example values
}

// Placeholder function to read an array of integers
fn read_array(n: usize) -> Vec<i32> {
    // Implementation to read an array of N integers from input
    // Example implementation (replace with actual input reading logic)
    vec![1, 2, 2, 3, 4] // Example values
}

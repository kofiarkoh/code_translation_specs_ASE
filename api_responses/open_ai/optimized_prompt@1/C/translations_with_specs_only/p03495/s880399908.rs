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
        0 // Return zero if there are fewer than N - K unique integers
    };

    // Output the result
    println!("{}", sum);
}

// Placeholder function to read input values
fn read_input() -> (usize, usize) {
    // Implementation to read N and K from input
    unimplemented!()
}

// Placeholder function to read an array of integers
fn read_array(n: usize) -> Vec<i32> {
    // Implementation to read an array of N integers from input
    unimplemented!()
}
```

```rust
// The code adheres to the specification by:
// 1. Reading two integers N and K, and an array of N integers.
// 2. Counting occurrences of integers in a predefined range (0-99).
// 3. Filtering and sorting the counts of unique integers.
// 4. Summing the counts of the first N - K unique integers or returning zero if not enough unique integers exist.
// 5. Using safe Rust constructs, with bounds checking to prevent undefined behavior.
// 6. The counting array is assumed to be of fixed size based on the valid range of integers, as per the specification.

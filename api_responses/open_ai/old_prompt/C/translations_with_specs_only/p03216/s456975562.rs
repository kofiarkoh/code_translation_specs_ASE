fn main() {
    // Example input values for testing
    let n: usize = 10; // Length of the string
    let s: &str = "DMDCCMDDMC"; // Input string
    let q: usize = 3; // Number of queries
    let k: Vec<usize> = vec![3, 5, 10]; // Array of lengths for each query

    // Process each query
    for &length in &k {
        // Ensure the length is valid
        if length > n {
            // Undefined behavior as per specification; we will just skip this case
            continue;
        }

        // Initialize counts
        let mut count_d = 0;
        let mut count_m = 0;
        let mut count_c = 0;

        // Count occurrences in the first `length` characters
        for ch in s.chars().take(length) {
            match ch {
                'D' => count_d += 1,
                'M' => count_m += 1,
                'C' => count_c += 1,
                _ => {} // Ignore other characters, though spec states only 'D', 'M', 'C' are present
            }
        }

        // Output the cumulative value for the current query
        let cumulative_value = count_d + count_m * count_c;
        println!("{}", cumulative_value);
    }
}

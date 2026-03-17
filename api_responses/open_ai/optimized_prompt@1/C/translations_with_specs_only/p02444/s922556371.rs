fn main() {
    use std::io::{self, BufRead};

    // Read the number of elements in the array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut array: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Extract start index, shift amount, and end index from the query
        let start = query[0];
        let shift = query[1];
        let end = query[2];

        // Calculate the length of the subarray
        let len = end - start + 1;

        // Perform circular shift only if the indices are valid
        if start < n && end < n && start <= end {
            // Normalize the shift amount to avoid unnecessary full rotations
            let effective_shift = shift % len;

            // Create a temporary vector to hold the shifted values
            let mut temp = array[start..=end].to_vec();

            // Perform the circular shift
            temp.rotate_right(effective_shift);

            // Place the shifted values back into the original array
            array[start..=end].copy_from_slice(&temp);
        }
    }

    // Output the modified array
    for value in array {
        print!("{} ", value);
    }
    println!(); // Print a newline at the end
}

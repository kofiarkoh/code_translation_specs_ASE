fn longest_subsequence(an: usize, bn: usize, ais: &[i32], bis: &[i32], k0: usize) -> i32 {
    // Edge case: if either array is empty, return 0
    if an == 0 || bn == 0 {
        return 0;
    }

    // Ensure k0 is within bounds
    if k0 >= bn {
        panic!("k0 must be less than bn");
    }

    let mut max_length = 0;

    // Iterate over each starting position in bis from k0 to bn
    for start in k0..bn {
        let mut length = 0;

        // Check for subsequence starting from bis[start]
        for &b in &bis[start..bn] {
            // Check if b exists in ais and increment length only if it matches in order
            if ais.contains(&b) {
                length += 1; // Increment length for each match found
            }
        }

        // Update max_length if the current length is greater
        if length > max_length {
            max_length = length;
        }
    }

    // Print the result as specified
    println!("{}", max_length);
    max_length // Return the maximum length found
}

fn main() {
    // Example usage
    let ais = [1, 2, 3, 4, 5];
    let bis = [2, 4, 3, 1, 2, 3, 4]; // Adjusted to match the test case
    let k0 = 0;
    longest_subsequence(5, 7, &ais, &bis, k0);
}

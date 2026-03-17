fn longest_subsequence(an: usize, bn: usize, ais: &[i32], bis: &[i32], k0: usize) -> i32 {
    // Check for edge cases where either array is empty
    if an == 0 || bn == 0 || k0 >= bn {
        return 0; // No subsequence can be formed
    }

    let mut max_length = 0;

    // Iterate over the starting index in `bis` from `k0` to `bn`
    for start in k0..bn {
        let mut length = 0;

        // Check for subsequence starting from `start` in `bis`
        for &b in &bis[start..] {
            // Check if the current element in `bis` exists in `ais`
            if ais.contains(&b) {
                length += 1; // Increment length if a match is found
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
    // Example usage (this part is not specified in the task but shows how to call the function)
    let ais = vec![1, 2, 3, 4, 5];
    let bis = vec![3, 1, 4, 2, 5];
    let an = ais.len();
    let bn = bis.len();
    let k0 = 0;

    longest_subsequence(an, bn, &ais, &bis, k0);
}

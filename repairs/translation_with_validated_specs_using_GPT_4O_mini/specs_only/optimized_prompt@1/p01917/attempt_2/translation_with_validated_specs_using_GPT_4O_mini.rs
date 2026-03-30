fn compute_min_difference(n: usize, x: &[i32], a: &[[i32; 110]; 3]) -> String {
    // Ensure the input meets the preconditions
    if n == 0 || n > 110 {
        return "Saiko".to_string(); // Undefined behavior for invalid input
    }

    let mut min_diff = i32::MAX; // Initialize minimum difference
    let mut valid_found = false; // Flag to check if a valid difference is found

    for i in 0..n {
        // Reset the array s for each iteration
        let mut s = [0; 3];

        for k in 0..3 {
            // Compute the transformation for each element in the input array
            let modified_value = a[k][i];
            let modified_value_plus_one = modified_value + 1;

            // Count how many elements exceed the modified value
            let count_exceeding = (0..n).filter(|&j| a[k][j] > modified_value).count();
            let count_exceeding_plus_one = (0..n).filter(|&j| a[k][j] > modified_value_plus_one).count();

            // Update s based on the counts
            s[k] = count_exceeding as i32;

            // Check if the count exceeds the limit of 7
            if count_exceeding <= 7 {
                let diff = modified_value - x[i];
                if diff < min_diff {
                    min_diff = diff;
                    valid_found = true; // A valid modification was found
                }
            }

            // Check for the modified value + 1
            if count_exceeding_plus_one <= 7 {
                let diff_plus_one = modified_value_plus_one - x[i];
                if diff_plus_one < min_diff {
                    min_diff = diff_plus_one;
                    valid_found = true; // A valid modification was found
                }
            }
        }
    }

    // Return the result based on whether a valid difference was found
    if valid_found {
        min_diff.to_string()
    } else {
        "Saiko".to_string()
    }
}

fn main() {
    // Example usage
    let n = 3;
    let x = [1, 2, 3];
    let a = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let result = compute_min_difference(n, &x, &a);
    println!("{}", result);
}

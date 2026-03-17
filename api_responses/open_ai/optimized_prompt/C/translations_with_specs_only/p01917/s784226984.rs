fn compute_min_difference(n: usize, values: &[i32], set1: &[i32], set2: &[i32], set3: &[i32], threshold: i32, limit: usize) -> Result<i32, &'static str> {
    // Preconditions check
    if n == 0 || values.len() != n || set1.len() != n || set2.len() != n || set3.len() != n {
        return Err("Invalid input");
    }

    let mut min_difference = i32::MAX; // Initialize to maximum possible value
    let mut found_valid_difference = false;

    // Iterate through each value in the input array
    for &value in values {
        // Attempt to modify the value with each set
        for &set_value in set1.iter().chain(set2.iter()).chain(set3.iter()) {
            let modified_value = value + set_value; // Modify the value

            // Count how many modified values exceed the threshold
            let count_exceeding = values.iter().filter(|&&v| v + set_value > threshold).count();

            // Check if the count exceeds the limit
            if count_exceeding <= limit {
                let difference = (modified_value - value).abs(); // Calculate the difference
                if difference < min_difference {
                    min_difference = difference; // Update minimum difference
                    found_valid_difference = true; // Mark that we found a valid difference
                }
            }
        }
    }

    // Return the result based on whether a valid difference was found
    if found_valid_difference {
        Ok(min_difference)
    } else {
        Err("Saiko") // No valid difference found
    }
}

// Example usage (commented out to avoid execution in a library context)
// fn main() {
//     let n = 5;
//     let values = [1, 2, 3, 4, 5];
//     let set1 = [1, 1, 1, 1, 1];
//     let set2 = [2, 2, 2, 2, 2];
//     let set3 = [3, 3, 3, 3, 3];
//     let threshold = 5;
//     let limit = 3;
//     match compute_min_difference(n, &values, &set1, &set2, &set3, threshold, limit) {
//         Ok(diff) => println!("Minimum difference: {}", diff),
//         Err(msg) => println!("{}", msg),
//     }
// }

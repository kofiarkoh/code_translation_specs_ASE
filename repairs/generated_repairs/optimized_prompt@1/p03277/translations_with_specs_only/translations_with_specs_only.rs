fn binary_search_threshold(size: usize, array: &[i32]) -> i32 {
    // Preconditions: size must be positive and array must contain at least one integer
    assert!(size > 0, "Size must be a positive integer.");
    assert!(!array.is_empty(), "Array must contain at least one integer.");

    // Define the lower and upper bounds for the binary search
    let mut lower_bound = i32::MIN; // Start from the minimum possible integer
    let mut upper_bound = i32::MAX; // Start from the maximum possible integer
    let target_count = (size as f32 / 4.0).ceil() as usize; // Calculate the target count

    // Function to count elements satisfying the condition
    fn count_satisfying_condition(array: &[i32], threshold: i32) -> usize {
        array.iter().filter(|&&x| x >= threshold).count()
    }

    // Perform binary search
    while lower_bound < upper_bound {
        let mid = (lower_bound + upper_bound + 1) / 2; // Upper mid to avoid infinite loop
        let count = count_satisfying_condition(array, mid);

        if count >= target_count {
            lower_bound = mid; // Move the lower bound up
        } else {
            upper_bound = mid - 1; // Move the upper bound down
        }
    }

    // Postcondition: return the largest threshold value found
    lower_bound
}

fn main() {
    // Example usage
    let size = 5; // Updated size to match the input
    let array = [1, 3, 2, 4, 5]; // Updated array to match the input
    let threshold = binary_search_threshold(size, &array);
    println!("{}", threshold); // Print only the threshold value
}

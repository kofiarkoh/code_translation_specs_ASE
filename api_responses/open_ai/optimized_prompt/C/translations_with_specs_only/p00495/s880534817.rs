fn longest_subsequence_length(arr1: &[i32], arr2: &[i32]) -> usize {
    let len1 = arr1.len();
    let len2 = arr2.len();
    
    // Initialize the maximum length found
    let mut max_length = 0;

    // Iterate through each starting position in the second array
    for start in 0..len2 {
        let mut current_length = 0;
        let mut j = start;

        // Check for subsequence starting from the current position in arr2
        for &value in arr1 {
            // Move j forward in arr2 to find the current value
            while j < len2 && arr2[j] != value {
                j += 1;
            }
            // If we found the value, increment the current length
            if j < len2 {
                current_length += 1;
                j += 1; // Move to the next position in arr2 for the next search
            }
        }

        // Update the maximum length found
        if current_length > max_length {
            max_length = current_length;
        }
    }

    // Return the maximum length found
    max_length
}

fn main() {
    // Example usage
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![3, 1, 2, 1, 3];
    
    let result = longest_subsequence_length(&arr1, &arr2);
    println!("{}", result); // Output the result
}

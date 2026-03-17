fn smallest_subarray_length(n: usize, k: usize, arr: &[i32]) -> i32 {
    // If N is less than K, return 0 immediately as per the specification.
    if n < k {
        return 0;
    }

    // A hashmap to count occurrences of integers from 1 to K in the current window.
    use std::collections::HashMap;
    let mut count = HashMap::new();
    let mut left = 0;
    let mut min_length = i32::MAX;
    let mut unique_count = 0;

    for right in 0..n {
        // Only consider integers from 1 to K for counting.
        if arr[right] >= 1 && arr[right] <= k {
            let entry = count.entry(arr[right]).or_insert(0);
            if *entry == 0 {
                unique_count += 1; // New unique integer found in the window.
            }
            *entry += 1; // Increment the count for this integer.
        }

        // While we have all integers from 1 to K in the current window, try to shrink it.
        while unique_count == k {
            // Update the minimum length if the current window is smaller.
            min_length = min_length.min((right - left + 1) as i32);

            // Try to remove the leftmost element from the window.
            if arr[left] >= 1 && arr[left] <= k {
                let entry = count.get_mut(&arr[left]).unwrap();
                *entry -= 1; // Decrement the count for this integer.
                if *entry == 0 {
                    unique_count -= 1; // We lost a unique integer from the window.
                }
            }
            left += 1; // Move the left pointer to shrink the window.
        }
    }

    // If min_length was updated, return it; otherwise, return 0.
    if min_length == i32::MAX {
        0
    } else {
        min_length
    }
}

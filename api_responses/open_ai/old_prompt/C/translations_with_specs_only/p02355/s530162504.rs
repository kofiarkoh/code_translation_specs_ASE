fn smallest_subarray_length(n: usize, k: usize, a: &[i32]) -> i32 {
    // If N < K, immediately return 0 as no valid subarray can exist
    if n < k {
        return 0;
    }

    // Frequency array to count occurrences of integers from 1 to K
    let mut freq = vec![0; k + 1];
    let mut count = 0; // Count of unique integers from 1 to K in the current window
    let mut min_length = n + 1; // Initialize to a value greater than any possible subarray length
    let (mut left, mut right) = (0, 0); // Sliding window pointers

    while right < n {
        // Expand the window by including the right element
        if a[right] >= 1 && a[right] <= k {
            if freq[a[right] as usize] == 0 {
                count += 1; // New unique integer found
            }
            freq[a[right] as usize] += 1; // Increment frequency
        }
        right += 1; // Move the right pointer

        // Contract the window from the left as long as we have all integers from 1 to K
        while count == k {
            // Update the minimum length if the current window is smaller
            min_length = min_length.min(right - left);
            // Try to reduce the window size from the left
            if a[left] >= 1 && a[left] <= k {
                freq[a[left] as usize] -= 1; // Decrement frequency
                if freq[a[left] as usize] == 0 {
                    count -= 1; // Lost a unique integer
                }
            }
            left += 1; // Move the left pointer
        }
    }

    // If min_length was updated, return it; otherwise, return 0
    if min_length == n + 1 {
        0
    } else {
        min_length as i32
    }
}

fn main() {
    // Example usage
    let n = 7;
    let k = 3;
    let a = vec![1, 2, 1, 3, 2, 1, 3];
    let result = smallest_subarray_length(n, k, &a);
    println!("{}", result); // Output the result
}

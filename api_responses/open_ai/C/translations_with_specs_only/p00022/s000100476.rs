use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of subsequent numbers
        if nums.is_empty() || nums[0] == 0 {
            break; // Termination condition
        }

        let n = nums[0] as usize; // Number of elements
        let arr = &nums[1..=n]; // The actual array of integers

        // Calculate the maximum sum of contiguous subarray
        let max_sum = max_contiguous_subarray_sum(arr);
        println!("{}", max_sum);
    }
}

fn max_contiguous_subarray_sum(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n == 0 {
        return 0; // Edge case, though per spec this shouldn't happen
    }

    let mut left_max = vec![0; n];
    let mut right_max = vec![0; n];

    // Calculate left max sums
    left_max[0] = arr[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(0) + arr[i];
    }

    // Calculate right max sums
    right_max[n - 1] = arr[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(0) + arr[i];
    }

    // Calculate the maximum sum of the contiguous subarray
    let mut max_sum = i32::MIN; // Start with the smallest possible value
    for i in 0..n {
        let current_sum = left_max[i] + right_max[i] - arr[i]; // Include arr[i] only once
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

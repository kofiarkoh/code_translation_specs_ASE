use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the first number is zero, which indicates termination
        if nums.is_empty() || nums[0] == 0 {
            break;
        }

        let n = nums.remove(0); // The first number indicates the count
        if n == 0 {
            continue; // Skip if count is zero
        }

        // Ensure we only process up to n integers
        if nums.len() > n as usize {
            nums.truncate(n as usize);
        }

        // Find the most frequent integer in the longest contiguous subarray
        let (most_frequent, max_length) = find_most_frequent(&nums);

        // Output the result based on the max_length
        if max_length > (n / 2) as usize { // Convert n / 2 to usize for comparison
            println!("{}", most_frequent);
        } else {
            println!("NO COLOR");
        }
    }
}

// Function to find the most frequent integer in the longest contiguous subarray
fn find_most_frequent(nums: &[i32]) -> (i32, usize) {
    let mut max_count = 0;
    let mut current_count = 1;
    let mut most_frequent = nums[0];
    let mut current_value = nums[0];

    for i in 1..nums.len() {
        if nums[i] == current_value {
            current_count += 1;
        } else {
            if current_count > max_count {
                max_count = current_count;
                most_frequent = current_value;
            }
            current_value = nums[i];
            current_count = 1;
        }
    }

    // Final check for the last contiguous segment
    if current_count > max_count {
        most_frequent = current_value;
        max_count = current_count;
    }

    (most_frequent, max_count)
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the number of elements
        if nums.is_empty() || nums[0] == 0 {
            break; // Termination condition
        }

        let n = nums[0] as usize; // Number of elements
        if n == 0 {
            continue; // Skip if no elements to process
        }

        // Extract the elements to sort
        let mut a = nums[1..n + 1].to_vec();
        let mut swap_count = 0;

        // Bubble sort implementation
        for u in (1..n).rev() {
            for i in 0..u {
                if a[i] > a[i + 1] {
                    a.swap(i, i + 1); // Swap elements
                    swap_count += 1; // Increment swap count
                }
            }
        }

        // Output the total number of swaps
        println!("{}", swap_count);
    }
}

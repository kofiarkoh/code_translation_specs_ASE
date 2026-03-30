use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first integer indicates the number of subsequent integers to be sorted.
        if nums.is_empty() {
            continue; // Skip empty lines
        }

        let count = nums[0];
        if count == 0 {
            break; // Termination condition
        }

        // Ensure we have the correct number of integers
        if nums.len() != (count as usize + 1) {
            continue; // Undefined behavior as per specification
        }

        // Extract the integers to be sorted
        let mut to_sort = nums[1..].to_vec();
        let swap_count = bubble_sort_and_count_swaps(&mut to_sort);

        // Output the total swap count
        println!("{}", swap_count);
    }
}

// Bubble sort implementation that counts swaps
fn bubble_sort_and_count_swaps(arr: &mut [i32]) -> i32 {
    let mut swap_count = 0;
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swap_count += 1; // Increment swap count
            }
        }
    }
    swap_count
}

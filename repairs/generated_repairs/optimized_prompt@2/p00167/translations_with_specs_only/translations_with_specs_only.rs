use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the size of the array
        if nums.is_empty() || nums[0] == 0 {
            break; // Termination condition
        }

        let size = nums[0] as usize;

        // Ensure the size is within the specified bounds
        if size > 100 || size + 1 > nums.len() {
            continue; // Ignore invalid sizes
        }

        // Extract the actual array elements
        let mut array = nums[1..=size].to_vec();
        let swap_count = bubble_sort_and_count_swaps(&mut array);
        
        // Output the number of swaps
        println!("{}", swap_count);
    }
}

// Bubble sort implementation that counts swaps
fn bubble_sort_and_count_swaps(arr: &mut Vec<i32>) -> usize {
    let mut swap_count = 0;
    let n = arr.len();
    
    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swap_count += 1; // Count the swap
            }
        }
    }
    
    swap_count
}

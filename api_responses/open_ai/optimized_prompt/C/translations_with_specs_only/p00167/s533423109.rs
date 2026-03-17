use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of elements
        if let Some(&count) = numbers.get(0) {
            if count == 0 {
                break; // Termination condition
            }
            // Ensure we only process valid sequences
            if count > 0 && count as usize == numbers.len() - 1 {
                let mut swap_count = 0;
                let mut arr = &mut numbers[1..]; // Exclude the count itself

                // Implementing a simple bubble sort to count swaps
                for i in 0..arr.len() {
                    for j in 0..arr.len() - 1 - i {
                        if arr[j] > arr[j + 1] {
                            arr.swap(j, j + 1);
                            swap_count += 1; // Count the swap
                        }
                    }
                }
                println!("{}", swap_count); // Output the swap count
            }
        }
    }
}

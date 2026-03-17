use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of subsequent integers
        if let Some(&n) = numbers.get(0) {
            if n <= 0 || n > 100 {
                // If n is not in the valid range, we terminate without output
                return;
            }

            // Ensure we only process the next n numbers
            let a = &mut numbers[1..(n as usize + 1)];
            let swap_count = bubble_sort(a);
            println!("{}", swap_count);
        }
    }
}

// Bubble Sort implementation that counts swaps
fn bubble_sort(arr: &mut [i32]) -> i32 {
    let mut swap_count = 0;
    let n = arr.len();
    let mut swapped;

    // Perform Bubble Sort
    for i in 0..n {
        swapped = false;
        for j in 0..(n - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swap_count += 1; // Count the swap
                swapped = true;
            }
        }
        // If no swaps occurred, the array is sorted
        if !swapped {
            break;
        }
    }
    swap_count
}

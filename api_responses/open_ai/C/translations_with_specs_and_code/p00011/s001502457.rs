use std::io::{self, BufRead};

fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read width
    let mut width = String::new();
    reader.read_line(&mut width).expect("Failed to read line");
    let width: usize = width.trim().parse().expect("Invalid width input");

    // Read number of swaps
    let mut num_swaps = String::new();
    reader.read_line(&mut num_swaps).expect("Failed to read line");
    let num_swaps: usize = num_swaps.trim().parse().expect("Invalid number of swaps input");

    // Initialize the list
    let mut list: Vec<i32> = (1..=width as i32).collect();

    // Perform swaps
    for _ in 0..num_swaps {
        let mut swap_input = String::new();
        reader.read_line(&mut swap_input).expect("Failed to read line");
        let indices: Vec<usize> = swap_input
            .trim()
            .split(',')
            .map(|s| s.parse().expect("Invalid index input"))
            .collect();

        // Ensure indices are within bounds
        if indices.len() == 2 {
            let a = indices[0];
            let b = indices[1];
            if a >= 1 && a <= width && b >= 1 && b <= width {
                swap(&mut list[a - 1], &mut list[b - 1]); // Convert to 0-based index
            }
        }
    }

    // Output the final list
    for &value in &list {
        println!("{}", value);
    }
}

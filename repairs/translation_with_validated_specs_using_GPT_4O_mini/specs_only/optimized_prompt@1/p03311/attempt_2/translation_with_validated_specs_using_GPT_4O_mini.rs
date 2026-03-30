use std::io::{self, BufRead};

fn main() {
    // Read the number of elements from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line as the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Input must be a positive integer");
    
    // Ensure n is positive
    if n == 0 {
        panic!("Number of elements must be positive");
    }

    // Read the sequence of long integers
    let mut sequence: Vec<i64> = lines
        .take(n)
        .map(|line| line.unwrap().parse().expect("Expected a long integer"))
        .collect();

    // Modify each element by subtracting its index + 1
    for (i, value) in sequence.iter_mut().enumerate() {
        *value -= i as i64 + 1; // Removed unnecessary parentheses
    }

    // Sort the modified sequence
    sequence.sort_unstable();

    // Initialize variables for minimum value calculation
    let mut sadness: i64 = sequence.iter().map(|&x| x - sequence[0]).sum();
    let mut level: i64 = sequence[0];
    let mut min_value: i64 = i64::MAX;

    // Iterate through the sorted sequence to compute the minimum value
    let mut i = 0;
    while i < n {
        let current = sequence[i];
        let mut count = 1;

        // Count how many times the current value appears
        while i + 1 < n && sequence[i + 1] == current {
            count += 1;
            i += 1;
        }

        // Move to the next unique element
        i += 1;

        // If there are no more elements, break
        if i >= n {
            break;
        }

        let next = sequence[i];
        let height_diff = next - current;

        // Check if we can update the level
        if height_diff * (n - i) as i64 <= sadness { // Cast n - i to i64
            level = next;
            sadness -= height_diff * count as i64;
        } else {
            // Calculate new level based on available sadness
            let new_level = current + sadness / (n - i) as i64; // Cast n - i to i64
            min_value = min_value.min(new_level);
            sadness = 0; // All sadness is used up
        }
    }

    // Final check for the minimum value
    min_value = min_value.min(level);

    // Print the computed minimum value
    println!("{}", min_value);
}

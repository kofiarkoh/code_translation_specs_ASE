use std::io::{self, BufRead};

fn main() {
    // Read the number of histogram bars
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of bars
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0), // Default to 0 if parsing fails
        _ => return, // Exit if there's an error reading the input
    };

    // Initialize a vector to hold the heights
    let mut heights = Vec::with_capacity(n);

    // Read the heights of the histogram bars
    for _ in 0..n {
        let height: usize = match lines.next() {
            Some(Ok(line)) => line.parse().unwrap_or(0), // Default to 0 if parsing fails
            _ => return, // Exit if there's an error reading the input
        };
        heights.push(height);
    }

    // Compute the maximum rectangular area
    let max_area = max_histogram_area(&heights);
    
    // Output the maximum area
    println!("{}", max_area);
}

// Function to compute the maximum rectangular area in a histogram
fn max_histogram_area(heights: &[usize]) -> usize {
    let mut max_area = 0;
    let mut stack = Vec::new();
    let mut index = 0;

    while index < heights.len() {
        // If this bar is higher than the bar at stack top, push it to the stack
        if stack.is_empty() || heights[index] >= heights[*stack.last().unwrap()] {
            stack.push(index);
            index += 1;
        } else {
            // Pop the top
            let top = stack.pop().unwrap();
            // Calculate the area with heights[top] as the smallest (or minimum height) bar 'h'
            let area = heights[top] * if stack.is_empty() {
                index
            } else {
                index - *stack.last().unwrap() - 1
            };
            // Update max area, if needed
            max_area = max_area.max(area);
        }
    }

    // Now pop the remaining bars from stack and calculate area with each popped bar
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let area = heights[top] * if stack.is_empty() {
            index
        } else {
            index - *stack.last().unwrap() - 1
        };
        max_area = max_area.max(area);
    }

    max_area
}

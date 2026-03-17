// This program computes the maximum rectangular area that can be formed in a histogram
// represented by a series of heights.

fn max_histogram_area(n: usize, heights: Vec<usize>) -> usize {
    // Preconditions: n must be non-negative and heights must contain non-negative integers.
    // If n is 0, the maximum area is 0 since there are no bars.
    if n == 0 {
        return 0;
    }

    // Stack to keep track of the indices of the histogram bars
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let mut i = 0;

    // Iterate through all bars of the histogram
    while i < n {
        // If this bar is higher than the bar at stack top, push it to the stack
        if stack.is_empty() || heights[stack.last().copied().unwrap()] <= heights[i] {
            stack.push(i);
            i += 1;
        } else {
            // Pop the top
            let top = stack.pop().unwrap();
            // Calculate the area with heights[top] as the smallest (or minimum height) bar 'h'
            let area = heights[top] * if stack.is_empty() {
                i // Width is i when stack is empty
            } else {
                i - stack.last().copied().unwrap() - 1 // Width is current index - index of last in stack - 1
            };
            // Update max area, if needed
            max_area = max_area.max(area);
        }
    }

    // Now pop the remaining bars from stack and calculate area with each popped bar
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let area = heights[top] * if stack.is_empty() {
            i // Width is i when stack is empty
        } else {
            i - stack.last().copied().unwrap() - 1 // Width is current index - index of last in stack - 1
        };
        max_area = max_area.max(area);
    }

    // Postcondition: The output is guaranteed to be non-negative
    max_area
}

// Example usage (not part of the specification):
// fn main() {
//     let heights = vec![2, 1, 5, 6, 2, 3];
//     let max_area = max_histogram_area(heights.len(), heights);
//     println!("Maximum rectangular area: {}", max_area);
// }

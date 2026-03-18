fn max_histogram_area(n: usize, heights: Vec<usize>) -> usize {
    // Ensure the number of bars is non-negative as per the specification
    if n == 0 {
        return 0; // If there are no bars, the area is 0
    }

    let mut max_area = 0; // To keep track of the maximum area found
    let mut stack: Vec<usize> = Vec::new(); // Stack to store indices of the histogram bars

    let mut i = 0;
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
                i // Width is i if stack is empty
            } else {
                i - stack.last().copied().unwrap() - 1 // Width is current index - index of last in stack - 1
            };
            // Update max area, if needed
            max_area = max_area.max(area);
        }
    }

    // Now pop the remaining bars from stack and calculate area
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let area = heights[top] * if stack.is_empty() {
            i // Width is i if stack is empty
        } else {
            i - stack.last().copied().unwrap() - 1 // Width is current index - index of last in stack - 1
        };
        max_area = max_area.max(area);
    }

    max_area // Return the maximum area found
}

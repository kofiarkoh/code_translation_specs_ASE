use std::io::{self, BufRead};

fn large_histogram(w: usize, table: &mut Vec<u64>) -> u64 {
    // Append a zero height to handle remaining bars in the stack
    table.push(0);
    let mut max_area = 0;
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..=w {
        // While the current height is less than the height of the bar at the top of the stack
        while let Some(&top) = stack.last() {
            if table[i] < table[top] {
                // Pop the top of the stack
                stack.pop();
                // Calculate the area with the popped height
                let height = table[top];
                // Determine the width
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                // Update max_area if needed
                max_area = max_area.max(height * width as u64);
            } else {
                break;
            }
        }
        // Push the current index onto the stack
        stack.push(i);
    }

    // Return the maximum area found
    max_area
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of bars
    let w: usize = lines.next().unwrap().unwrap().parse().unwrap();
    // Initialize the table with the heights
    let mut table: Vec<u64> = lines
        .take(w)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Call the function to calculate the maximum area
    let max_area = large_histogram(w, &mut table);
    // Print the result
    println!("{}", max_area);
}

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
                let height = table[top];
                stack.pop();
                // Calculate the width
                let width = if stack.is_empty() {
                    i // If stack is empty, width is i
                } else {
                    i - stack.last().unwrap() - 1 // Width is the distance to the next lower bar
                };
                // Calculate area and update max_area
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

    // Read the number of bars in the histogram
    let w: usize = lines.next().unwrap().unwrap().parse().unwrap();
    // Initialize the table with the heights
    let mut table: Vec<u64> = lines
        .take(w)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Ensure the table has valid heights
    assert!(w <= 100_000);
    for &height in &table {
        assert!(height >= 0);
    }

    // Calculate the maximum rectangular area
    let max_area = large_histogram(w, &mut table);

    // Print the maximum area
    println!("{}", max_area);
}

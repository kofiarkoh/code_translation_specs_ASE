use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.expect("Failed to read line");
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid integer"))
            .collect();

        if dimensions.len() != 2 {
            continue; // Skip invalid input
        }

        let (height, width) = (dimensions[0], dimensions[1]);
        let mut grid = Vec::with_capacity(height);

        for _ in 0..height {
            if let Some(line) = lines.next() {
                let line = line.expect("Failed to read line");
                grid.push(line.chars().collect::<Vec<_>>());
            }
        }

        let max_area = max_rectangle_area(&grid);
        println!("{}", max_area);
    }
}

fn max_rectangle_area(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();
    let mut heights = vec![0; width];
    let mut max_area = 0;

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == '.' {
                heights[col] += 1; // Increase height for '.' character
            } else {
                heights[col] = 0; // Reset height for non '.' character
            }
        }
        max_area = max_area.max(largest_rectangle_area(&heights));
    }

    max_area
}

fn largest_rectangle_area(heights: &[usize]) -> usize {
    let mut max_area = 0;
    let mut stack = Vec::new();
    let mut extended_heights = heights.to_vec();
    extended_heights.push(0); // Sentinel value to flush the stack

    for (i, &h) in extended_heights.iter().enumerate() {
        while let Some(&last_index) = stack.last() {
            if h < extended_heights[last_index] {
                let height = extended_heights[last_index];
                stack.pop();
                let width = if stack.is_empty() {
                    i
                } else {
                    i - stack.last().unwrap() - 1
                };
                max_area = max_area.max(height * width);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
}

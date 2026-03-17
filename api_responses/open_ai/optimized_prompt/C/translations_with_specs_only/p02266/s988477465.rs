use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    let (total_area, pool_count, pool_areas) = calculate_pools(&input);
    
    // Output the total area, count of pools, and areas in reverse order
    println!("{}", total_area);
    println!("{}", pool_count);
    for area in pool_areas.iter().rev() {
        println!("{}", area);
    }
}

fn calculate_pools(input: &str) -> (usize, usize, Vec<usize>) {
    let mut stack: Vec<usize> = Vec::new(); // Stack to track heights
    let mut pool_areas: Vec<usize> = Vec::new(); // List to track areas of pools
    let mut total_area = 0;
    let mut pool_count = 0;

    let mut current_height = 0;

    for c in input.chars() {
        match c {
            '\\' => {
                // Push current height onto the stack
                stack.push(current_height);
                current_height = 0; // Reset height for the next segment
            }
            '_' => {
                // Increase height for the current segment
                current_height += 1;
            }
            '/' => {
                // If we encounter '/', we need to calculate the pool
                if let Some(last_height) = stack.pop() {
                    let pool_area = current_height + last_height; // Area of the pool
                    total_area += pool_area;
                    pool_count += 1;
                    pool_areas.push(pool_area);
                    current_height = last_height; // Set current height to last height
                }
            }
            _ => {
                // Invalid characters are ignored as per the specification
                continue;
            }
        }
    }

    (total_area, pool_count, pool_areas)
}

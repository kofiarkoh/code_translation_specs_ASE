use std::io::{self, Write};

fn main() {
    loop {
        // Read dimensions X and Y
        let (x, y) = read_dimensions();
        
        // Termination condition
        if x == 0 && y == 0 {
            break;
        }

        // Read course data
        let course_data = read_course_data(x, y);
        
        // Compute the sum based on the course data
        let result = compute_sum(&course_data, x, y);
        
        // Print the result
        println!("{}", result);
    }
}

// Function to read dimensions from standard input
fn read_dimensions() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let dimensions: Vec<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // Ensure we have exactly two dimensions
    if dimensions.len() == 2 {
        (dimensions[0], dimensions[1])
    } else {
        (0, 0) // Return (0, 0) if input is invalid
    }
}

// Function to read course data based on dimensions
fn read_course_data(x: usize, y: usize) -> Vec<Vec<i32>> {
    let mut course_data = vec![vec![0; y]; x];
    
    for i in 0..x {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        // Ensure the row has the correct number of columns
        if row.len() == y {
            course_data[i] = row;
        }
    }
    
    course_data
}

// Function to compute the sum based on the course data
fn compute_sum(course_data: &[Vec<i32>], x: usize, y: usize) -> i32 {
    let mut total_sum = 0;

    for i in 0..x {
        for j in 0..y {
            match course_data[i][j] {
                0 => {
                    // Count contributions from this cell
                    total_sum += count_contributions(course_data, i, j, x, y);
                }
                1 => {
                    // Reset contributions from this path
                    // No action needed as contributions are handled in count_contributions
                }
                2 => {
                    // Allow for additional contributions
                    total_sum += count_contributions(course_data, i, j, x, y);
                }
                _ => {}
            }
        }
    }

    total_sum
}

// Function to count contributions based on neighboring cells
fn count_contributions(course_data: &[Vec<i32>], i: usize, j: usize, x: usize, y: usize) -> i32 {
    let mut contribution = 0;

    // Check neighboring cells (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (di, dj) in directions.iter() {
        let ni = i as isize + di;
        let nj = j as isize + dj;

        if ni >= 0 && ni < x as isize && nj >= 0 && nj < y as isize {
            match course_data[ni as usize][nj as usize] {
                0 => contribution += 1, // Count contribution from 0
                1 => contribution = 0,   // Reset contribution if 1 is found
                2 => contribution += 2,   // Additional contribution from 2
                _ => {}
            }
        }
    }

    contribution
}

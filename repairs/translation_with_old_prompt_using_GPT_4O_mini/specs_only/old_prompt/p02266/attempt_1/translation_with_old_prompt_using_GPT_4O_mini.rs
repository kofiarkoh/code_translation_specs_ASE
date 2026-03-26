use std::io::{self, BufRead};

struct Pool {
    area: usize,
    next: Option<Box<Pool>>,
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    let mut stack: Vec<usize> = Vec::new();
    let mut total_area = 0;
    let mut pool_count = 0;
    let mut pool_areas: Vec<usize> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        match c {
            '\\' => stack.push(i),
            '/' => {
                if let Some(start) = stack.pop() {
                    let area = i - start; // Calculate area between the two elevations
                    total_area += area;
                    pool_count += 1;
                    pool_areas.push(area);
                }
            }
            '_' => {
                // '_' does not affect the stack but can be part of a pool
                // No action needed here as it does not change the state
            }
            _ => unreachable!(), // According to the spec, input is well-formed
        }
    }

    // Output total area and count of pools
    println!("{} {}", total_area, pool_count);
    // Output areas of each pool
    for area in pool_areas {
        println!("{}", area);
    }
}

use std::io::{self, BufRead};

struct Pool {
    area: usize,
    next: Option<Box<Pool>>,
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    let mut stack: Vec<usize> = Vec::new();
    let mut pools: Option<Box<Pool>> = None;
    let mut total_area = 0;
    let mut pool_count = 0;

    for ch in input.chars() {
        match ch {
            '\\' => {
                // Start a new pool
                stack.push(1); // Start a new pool with area 1
            }
            '/' => {
                // End the current pool
                if let Some(value) = stack.pop() {
                    let area = value * 2; // Calculate area based on stack value
                    total_area += area;
                    pool_count += 1;

                    // Create a new pool and link it
                    let new_pool = Box::new(Pool {
                        area,
                        next: pools.take(),
                    });
                    pools = Some(new_pool);
                }
            }
            _ => {
                // Ignore other characters
            }
        }
    }

    // Output total area and pool count
    println!("{}", total_area);
    println!("{}", pool_count);

    let mut current_pool = pools;
    let mut areas = Vec::new();
    while let Some(pool) = current_pool {
        areas.push(pool.area);
        current_pool = pool.next;
    }
    
    // Print areas in the correct order
    for area in areas {
        println!("{}", area);
    }
}

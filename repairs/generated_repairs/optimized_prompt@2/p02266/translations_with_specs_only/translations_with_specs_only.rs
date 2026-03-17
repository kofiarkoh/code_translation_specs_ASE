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
            '_' => {
                // Start a new pool
                stack.push(1); // Start a new pool with area 1
            }
            '/' => {
                // End the current pool
                if let Some(value) = stack.pop() {
                    let area = value; // Area is the value popped from the stack
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

    // Output total pool count and total area
    println!("{}", pool_count);
    println!("{}", total_area);

    let mut current_pool = pools;
    let mut areas = Vec::new();
    while let Some(pool) = current_pool {
        areas.push(pool.area);
        current_pool = pool.next;
    }
    
    for area in areas.iter().rev() {
        println!("{}", area);
    }
}

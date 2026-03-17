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
            // Assuming '_' starts a new pool and '/' ends it
            '_' => {
                // Push a new value onto the stack
                stack.push(1); // Example value, can be adjusted based on actual logic
            }
            '/' => {
                // Pop from the stack and calculate pool area
                if let Some(value) = stack.pop() {
                    let area = value * 2; // Example area calculation
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

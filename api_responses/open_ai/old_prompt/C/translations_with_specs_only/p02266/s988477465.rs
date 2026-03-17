use std::io::{self, BufRead};

struct Pool {
    area: i32,
    next: Option<Box<Pool>>,
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    let mut stack: Vec<i32> = Vec::new();
    let mut pools: Option<Box<Pool>> = None;
    let mut total_area = 0;
    let mut pool_count = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '\\' => {
                stack.push(i as i32);
            }
            '/' => {
                if let Some(start) = stack.pop() {
                    let area = i as i32 - start;
                    total_area += area;
                    pool_count += 1;

                    // Create a new pool and add it to the linked list
                    let new_pool = Pool { area, next: pools };
                    pools = Some(Box::new(new_pool));
                }
            }
            '_' => {
                // '_' does not affect the stack or pools, just a flat area
                continue;
            }
            _ => {
                // According to the specification, we assume input is well-formed
                // and does not contain invalid characters.
                continue;
            }
        }
    }

    // Output total area and count of pools
    println!("{} {}", total_area, pool_count);

    // Output areas of each pool
    let mut current_pool = pools;
    while let Some(pool) = current_pool {
        println!("{}", pool.area);
        current_pool = pool.next;
    }
}

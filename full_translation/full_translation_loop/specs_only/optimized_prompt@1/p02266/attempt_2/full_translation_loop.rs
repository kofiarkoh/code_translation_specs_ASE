use std::io::{self, BufRead};

struct Pool {
    area: usize,
    next: Option<Box<Pool>>,
}

struct Stack {
    values: Vec<usize>,
}

impl Stack {
    fn new() -> Self {
        Stack { values: Vec::new() }
    }

    fn push(&mut self, value: usize) {
        self.values.push(value);
    }

    fn pop(&mut self) -> Option<usize> {
        self.values.pop()
    }

    fn top(&self) -> Option<usize> {
        self.values.last().copied()
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().filter_map(Result::ok).collect::<String>();

    let mut stack = Stack::new();
    let mut pools: Vec<usize> = Vec::new();
    let mut total_area = 0;

    let mut current_area = 0;

    for ch in input.chars() {
        match ch {
            '\\' => {
                // Start of a pool
                if let Some(value) = stack.pop() {
                    current_area += value; // Add to current area
                }
            }
            '/' => {
                // End of a pool
                if current_area > 0 {
                    pools.push(current_area);
                    total_area += current_area;
                    current_area = 0; // Reset for the next pool
                }
            }
            '_' => {
                // Represents a unit of area
                stack.push(1); // Each '_' contributes 1 to the area
            }
            _ => {
                // Ignore other characters
            }
        }
    }

    // Output total area and count of pools
    let pool_count = pools.len();
    println!("{}", total_area);
    println!("{}", pool_count);

    // Output individual areas in reverse order
    for area in pools.iter().rev() {
        println!("{}", area);
    }
}

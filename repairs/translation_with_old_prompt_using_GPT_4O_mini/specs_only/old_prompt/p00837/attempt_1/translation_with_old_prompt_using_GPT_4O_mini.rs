use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read parameters
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Number of desks
    let c: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Capacity of each desk
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Number of students

    // Initialize desk states
    let mut desks: Vec<usize> = vec![0; m]; // Number of books on each desk
    let mut total_movements = 0;

    // Process each student's request
    for _ in 0..n {
        let request: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let t = request[0]; // Number of books requested
        let id = request[1]; // Book ID (not used in logic)

        // Simulate the allocation of books
        let mut books_to_allocate = t;
        for desk in &mut desks {
            if books_to_allocate == 0 {
                break;
            }
            let available_space = c - *desk;
            if available_space > 0 {
                let allocation = available_space.min(books_to_allocate);
                *desk += allocation;
                books_to_allocate -= allocation;
                total_movements += allocation; // Count movements as books are allocated
            }
        }
    }

    // Output the total number of desk movements
    println!("{}", total_movements);
}

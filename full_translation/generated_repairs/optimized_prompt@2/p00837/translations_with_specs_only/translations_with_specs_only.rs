// Book replacement system for students using a fixed number of desks
// This program processes requests for books and calculates the total cost based on desk assignments.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read each line of input
    for line in reader.lines() {
        let line = line.unwrap();
        let mut inputs = line.split_whitespace();
        
        // Read the number of desks, maximum capacity, and number of students
        let num_desks: usize = inputs.next().unwrap().parse().unwrap();
        let max_capacity: usize = inputs.next().unwrap().parse().unwrap();
        let num_students: usize = inputs.next().unwrap().parse().unwrap();
        
        // Check preconditions
        if num_desks == 0 || max_capacity == 0 || num_students == 0 {
            // Terminate if any input is non-positive
            return;
        }
        
        // Initialize desks
        let mut desks: Vec<Vec<usize>> = vec![Vec::new(); num_desks];
        let mut total_cost = 0;

        // Process each student's book requests
        for _ in 0..num_students {
            let mut book_requests: Vec<usize> = Vec::new();
            // Read book requests until the end of the line
            let request_line = line.split_whitespace().skip(3).collect::<Vec<&str>>().join(" ");
            for book_id in request_line.split_whitespace() {
                if let Ok(id) = book_id.parse() {
                    book_requests.push(id);
                }
            }
            
            // Sort book requests to minimize cost (as per non-functional constraints)
            book_requests.sort();

            // Allocate books to desks
            for book_id in book_requests {
                // Find a desk to place the book
                for desk in &mut desks {
                    if desk.len() < max_capacity {
                        desk.push(book_id);
                        total_cost += 1; // Increment cost for each book placed
                        break; // Move to the next book request
                    }
                }
            }
        }

        // Output the total cost for the current test case
        println!("{}", total_cost);
    }
}

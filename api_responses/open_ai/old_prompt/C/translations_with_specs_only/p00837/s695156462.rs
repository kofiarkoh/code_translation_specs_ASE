use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of desks, capacity, number of students
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut params = input.split_whitespace();
    let m: usize = params.next().unwrap().parse().unwrap(); // Number of desks
    let c: usize = params.next().unwrap().parse().unwrap(); // Capacity of each desk
    let n: usize = params.next().unwrap().parse().unwrap(); // Number of students

    // Initialize desk states
    let mut desks: Vec<Vec<i32>> = vec![Vec::new(); m]; // Each desk can hold books
    let mut total_movements = 0;

    // Process each student's requests
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut requests = input.split_whitespace();
        let t: usize = requests.next().unwrap().parse().unwrap(); // Number of books requested

        for _ in 0..t {
            let id: i32 = requests.next().unwrap().parse().unwrap(); // Book ID

            // Find a desk to place the book
            let mut placed = false;
            for desk in &mut desks {
                if desk.len() < c {
                    desk.push(id); // Place book on desk
                    placed = true;
                    break;
                }
            }

            // If no desk could accommodate the book, we need to move books
            if !placed {
                // Move a book from the first desk to make space
                if let Some(first_desk) = desks.get_mut(0) {
                    if let Some(moved_book) = first_desk.pop() {
                        total_movements += 1; // Count the movement
                        // Place the new book on the now available desk
                        first_desk.push(id);
                    }
                }
            }
        }
    }

    // Output the total number of desk movements
    println!("{}", total_movements);
}

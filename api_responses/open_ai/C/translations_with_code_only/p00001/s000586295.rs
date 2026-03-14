use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut rank = [0; 3]; // Initialize rank array with zeros

    for _ in 0..10 {
        let mut work = String::new();
        // Read input line and parse it as an integer
        handle.read_line(&mut work).expect("Failed to read line");
        let work: i32 = work.trim().parse().expect("Failed to parse integer");

        // Update the rank array based on the value of work
        if work > rank[0] {
            rank[2] = rank[1];
            rank[1] = rank[0];
            rank[0] = work;
        } else if work > rank[1] {
            rank[2] = rank[1];
            rank[1] = work;
        } else if work > rank[2] {
            rank[2] = work;
        }
    }

    // Print the ranks
    for &r in &rank {
        println!("{}", r);
    }
}

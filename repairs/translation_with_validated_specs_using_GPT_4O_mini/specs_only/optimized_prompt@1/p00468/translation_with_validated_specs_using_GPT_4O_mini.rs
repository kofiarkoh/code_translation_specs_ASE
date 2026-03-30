use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read all lines from the input at once
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut i = 0;
    while i < lines.len() {
        let mut iter = lines[i].split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Terminate gracefully if both n and m are zero
        if n == 0 && m == 0 {
            break;
        }

        // Initialize the connection matrix
        let mut connections = vec![vec![false; n]; n];

        // Read the connections
        for _ in 0..m {
            i += 1; // Move to the next line for connections
            let line = &lines[i];
            let mut iter = line.split_whitespace();
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            // Adjust for 0-based indexing
            connections[a - 1][b - 1] = true;
            connections[b - 1][a - 1] = true;
        }

        // Use a set to track unique connected students
        let mut visited = HashSet::new();
        let mut stack = vec![0]; // Start with the first student (index 0)

        // Depth-first search to find all connected students
        while let Some(student) = stack.pop() {
            if visited.insert(student) {
                for neighbor in 0..n {
                    if connections[student][neighbor] && !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        // Output the count of unique connected students excluding the first student
        // The first student is represented by index 0, so we subtract 1 from the count
        println!("{}", visited.len() - 1);
        i += 1; // Move to the next dataset
    }
}

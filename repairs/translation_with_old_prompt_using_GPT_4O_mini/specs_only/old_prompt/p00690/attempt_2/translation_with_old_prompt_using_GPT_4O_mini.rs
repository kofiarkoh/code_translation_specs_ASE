use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Connection {
    a: usize,
    b: usize,
    d: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read number of stations and connections
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = parts[0]; // number of stations
    let m = parts[1]; // number of connections

    // Read connections
    let mut tbl: Vec<Connection> = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        tbl.push(Connection {
            a: parts[0],
            b: parts[1],
            d: parts[2],
        });
    }

    // Variables to track the longest path
    let mut max_distance = 0;
    let mut best_path = Vec::new();

    // Function to explore paths recursively
    fn explore(
        current: usize,
        distance: usize,
        path: &mut Vec<usize>,
        tbl: &[Connection], // Changed to immutable borrow
        max_distance: &mut usize,
        best_path: &mut Vec<usize>,
        visited: &mut HashMap<usize, bool>,
    ) {
        // Mark the current station as visited
        visited.insert(current, true);
        path.push(current);

        // Check for the longest path
        if distance > *max_distance {
            *max_distance = distance;
            *best_path = path.clone();
        }

        // Explore connections
        for conn in tbl.iter() {
            if conn.a == current && !visited.get(&conn.b).unwrap_or(&false) {
                explore(conn.b, distance + conn.d, path, tbl, max_distance, best_path, visited);
            }
        }

        // Backtrack
        path.pop();
        visited.insert(current, false);
    }

    // Start exploring from each station
    for i in 1..=n {
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        explore(i, 0, &mut path, &tbl, &mut max_distance, &mut best_path, &mut visited); // Pass tbl as immutable
    }

    // Output the results
    println!("{}", max_distance);
    println!("{}", best_path.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of stations and connections
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0]; // Number of stations
    let m = parts[1]; // Number of connections

    // Early exit if there are no stations
    if n == 0 {
        return;
    }

    // Read the connections
    let mut tbl: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let a = parts[0]; // Starting station
        let b = parts[1]; // Ending station
        let d = parts[2]; // Distance
        tbl.push((a, b, d));
    }

    // To store the longest path length and the corresponding path
    let mut max_length = 0;
    let mut best_path = Vec::new();

    // Function to explore paths recursively
    fn explore(
        current: usize,
        current_length: usize,
        visited: &mut HashMap<usize, bool>,
        tbl: &[(usize, usize, usize)],
        max_length: &mut usize,
        best_path: &mut Vec<usize>,
        path: &mut Vec<usize>,
    ) {
        // Mark the current station as visited
        visited.insert(current, true);
        path.push(current);

        // Check for the longest path
        if current_length > *max_length {
            *max_length = current_length;
            *best_path = path.clone();
        }

        // Explore connections
        for &(a, b, d) in tbl {
            if a == current && !visited.contains_key(&b) {
                explore(b, current_length + d, visited, tbl, max_length, best_path, path);
            }
        }

        // Backtrack
        visited.remove(&current);
        path.pop();
    }

    // Start exploring from each station
    for i in 1..=n {
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        explore(i, 0, &mut visited, &tbl, &mut max_length, &mut best_path, &mut path);
    }

    // Output the results
    println!("{}", max_length);
    println!("{}", best_path.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}

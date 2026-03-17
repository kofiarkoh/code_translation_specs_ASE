use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        
        // Read n (number of students) and m (number of connections)
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        // Initialize the connection matrix
        let mut connections = vec![vec![false; n]; n];

        // Read m pairs of connections
        for _ in 0..m {
            let line = reader.lines().next().unwrap().unwrap();
            let mut pair_iter = line.split_whitespace();
            let p1: usize = pair_iter.next().unwrap().parse().unwrap();
            let p2: usize = pair_iter.next().unwrap().parse().unwrap();

            // Adjust for 0-based indexing
            let p1_index = p1 - 1;
            let p2_index = p2 - 1;

            // Mark the connections as bidirectional
            if p1_index < n && p2_index < n {
                connections[p1_index][p2_index] = true;
                connections[p2_index][p1_index] = true;
            }
        }

        // Count connected students using DFS
        let mut visited = vec![false; n];
        let count = dfs(0, &connections, &mut visited);

        // Output the count of connected students
        println!("{}", count);
    }
}

// Depth-First Search to count connected students
fn dfs(student: usize, connections: &Vec<Vec<bool>>, visited: &mut Vec<bool>) -> usize {
    if visited[student] {
        return 0; // Already visited
    }
    visited[student] = true; // Mark as visited

    // Count this student
    let mut count = 1;

    // Visit all connected students
    for (i, &is_connected) in connections[student].iter().enumerate() {
        if is_connected {
            count += dfs(i, connections, visited);
        }
    }

    count
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Edge case: If n is 0, we terminate without output
    if n == 0 {
        return;
    }

    // Initialize costs and visited flags
    let mut costs = vec![-1; n]; // -1 indicates unreachable
    let mut visited = vec![false; n];
    costs[0] = 0; // Starting node cost is 0

    // Read the graph
    let mut adjacency_list: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let u = parts[0];
        let k = parts[1];
        
        for j in 0..k {
            let edge_line = lines.next().unwrap().unwrap();
            let edge_parts: Vec<usize> = edge_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let to = edge_parts[0];
            let cost = edge_parts[1];
            adjacency_list[u].push((to, cost));
        }
    }

    // Dijkstra's algorithm without using a heap
    for _ in 0..n {
        // Find the unvisited node with the smallest cost
        let mut min_cost = usize::MAX;
        let mut min_index = None;

        for i in 0..n {
            if !visited[i] && costs[i] != -1 && costs[i] < min_cost {
                min_cost = costs[i];
                min_index = Some(i);
            }
        }

        // If no unvisited node is reachable, break
        let current_index = match min_index {
            Some(index) => index,
            None => break,
        };

        visited[current_index] = true;

        // Update costs for neighbors
        for &(neighbor, edge_cost) in &adjacency_list[current_index] {
            let new_cost = costs[current_index] + edge_cost;
            if costs[neighbor] == -1 || new_cost < costs[neighbor] {
                costs[neighbor] = new_cost;
            }
        }
    }

    // Output the results
    for i in 0..n {
        println!("{} {}", i, costs[i]);
    }
}

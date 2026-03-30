use std::collections::{VecDeque, HashMap};

fn bfs_shortest_path(n: usize, m: usize, edges: &[(usize, usize)]) -> (String, Vec<i32>) {
    // Initialize predecessor array with -1 for unvisited nodes
    let mut predecessor = vec![-1; n + 1];
    
    // Create an adjacency list for the graph
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(u, v) in edges {
        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u);
    }

    // BFS initialization
    let mut queue = VecDeque::new();
    queue.push_back(1); // Start from node 1
    predecessor[1] = 0; // Mark the starting node's predecessor as 0 (or any sentinel value)

    // BFS traversal
    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                // Only visit unvisited nodes
                if predecessor[neighbor] == -1 {
                    predecessor[neighbor] = node as i32; // Record predecessor
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Check if the graph is connected
    let is_connected = predecessor.iter().skip(2).all(|&p| p != -1);
    let result = if is_connected { "Yes".to_string() } else { "No".to_string() };

    // Return the result and the predecessor array
    (result, predecessor)
}

// Example usage (commented out to avoid execution in a library context)
// fn main() {
//     let n = 5;
//     let m = 4;
//     let edges = vec![(1, 2), (1, 3), (2, 4), (3, 5)];
//     let (result, predecessors) = bfs_shortest_path(n, m, &edges);
//     println!("{}", result);
//     for i in 2..=n {
//         println!("Predecessor of node {}: {}", i, predecessors[i]);
//     }
// }

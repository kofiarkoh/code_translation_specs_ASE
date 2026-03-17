use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of nodes (N) and edges (M)
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the graph as an adjacency list
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    
    // Read the edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let a: usize = edge_parts.next().unwrap().parse().unwrap();
        let b: usize = edge_parts.next().unwrap().parse().unwrap();
        
        // Add the undirected edges
        graph.entry(a).or_insert_with(Vec::new).push(b);
        graph.entry(b).or_insert_with(Vec::new).push(a);
    }

    // Initialize the predecessor array with -1
    let mut d = vec![-1; n + 1];
    d[1] = 0; // Starting node's predecessor is set to 0

    // BFS initialization
    let mut queue = VecDeque::new();
    queue.push_back(1); // Start BFS from node 1

    // Perform BFS
    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if d[neighbor] == -1 { // If not visited
                    d[neighbor] = node as isize; // Set predecessor
                    queue.push_back(neighbor); // Enqueue the neighbor
                }
            }
        }
    }

    // Output results
    println!("Yes");
    for i in 2..=n {
        println!("{}", d[i]);
    }
}

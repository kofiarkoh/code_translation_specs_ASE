use std::io::{self, BufRead};

struct Node {
    cost: i32,
    visited: bool,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Handle the case where n is 0
    if n == 0 {
        return; // No output for zero nodes
    }

    // Initialize nodes
    let mut nodes: Vec<Node> = vec![Node { cost: -1, visited: false }; n];
    nodes[0].cost = 0; // Starting node cost is 0

    // Read the graph edges
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..k {
            let to: usize = parts.next().unwrap().parse().unwrap();
            let cost: i32 = parts.next().unwrap().parse().unwrap();
            // Update the cost if the edge leads to a shorter path
            if nodes[u].cost != -1 {
                let new_cost = nodes[u].cost + cost;
                if nodes[to].cost == -1 || new_cost < nodes[to].cost {
                    nodes[to].cost = new_cost;
                }
            }
        }
    }

    // Output the shortest path costs
    for (index, node) in nodes.iter().enumerate() {
        println!("{} {}", index, node.cost);
    }
}

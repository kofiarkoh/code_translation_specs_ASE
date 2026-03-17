use std::io::{self, BufRead};

#[derive(Debug)]
struct Edge {
    capacity: i32,
    cost: i32,
    flow: i32,
}

#[derive(Debug)]
struct Graph {
    nodes: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Graph {
            nodes,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, capacity: i32, cost: i32) {
        self.edges.push(Edge {
            capacity,
            cost,
            flow: 0,
        });
    }

    fn min_cost_flow(&mut self, source: usize, sink: usize) -> i32 {
        // Placeholder for the actual min-cost flow algorithm
        // This should implement the logic to compute the minimum cost flow
        // For now, we return -1 to indicate failure as per the specification
        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        
        // Read number of nodes (N) and edges (M)
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        // Create a new graph
        let mut graph = Graph::new(n);

        // Read edges
        for _ in 0..m {
            let edge_line = reader.lines().next().unwrap().unwrap();
            let mut edge_parts = edge_line.split_whitespace();
            let capacity: i32 = edge_parts.next().unwrap().parse().unwrap();
            let cost: i32 = edge_parts.next().unwrap().parse().unwrap();
            graph.add_edge(capacity, cost);
        }

        // Compute minimum cost flow from source (0) to sink (n-1)
        let min_cost = graph.min_cost_flow(0, n - 1);
        
        // Output the result
        println!("{}", min_cost);
    }
}

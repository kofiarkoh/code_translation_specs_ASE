use std::io::{self, BufRead};
use std::fmt;

#[derive(Debug)]
struct Graph {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new(n: usize, m: usize, edges: Vec<(usize, usize)>) -> Self {
        Graph { n, m, edges }
    }

    fn average_distance(&self) -> f64 {
        let mut distances = vec![f64::INFINITY; self.n];
        distances[0] = 0.0; // Starting from node 0

        // Bellman-Ford-like relaxation for average distance calculation
        for _ in 0..self.n - 1 {
            for &(u, v) in &self.edges {
                if distances[u] != f64::INFINITY {
                    distances[v] = distances[v].min(distances[u] + 1.0);
                }
            }
        }

        // Calculate average distance
        let total_distance: f64 = distances.iter().filter(|&&d| d < f64::INFINITY).sum();
        let reachable_nodes = distances.iter().filter(|&&d| d < f64::INFINITY).count();
        
        if reachable_nodes == 0 {
            return 0.0; // Avoid division by zero
        }

        total_distance / reachable_nodes as f64
    }

    fn minimum_average_distance(&self) -> f64 {
        let original_avg = self.average_distance();
        let mut min_avg = original_avg;

        // Evaluate the effect of removing each edge
        for &(u, v) in &self.edges {
            let mut modified_edges = self.edges.clone();
            modified_edges.retain(|&edge| edge != (u, v));
            let modified_graph = Graph::new(self.n, self.m - 1, modified_edges);
            let new_avg = modified_graph.average_distance();
            min_avg = min_avg.min(new_avg);
        }

        min_avg
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let m: usize = params.next().unwrap().parse().unwrap();

    let mut edges = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut edge_params = line.split_whitespace();
        let i: usize = edge_params.next().unwrap().parse().unwrap();
        let j: usize = edge_params.next().unwrap().parse().unwrap();
        edges.push((i, j));
    }

    let graph = Graph::new(n, m, edges);
    let min_avg_distance = graph.minimum_average_distance();

    // Print the result formatted to ten decimal places
    println!("{:.10}", min_avg_distance);
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

const MAX_NODES: usize = 100;
const INF: usize = usize::MAX;

#[derive(Debug)]
struct Graph {
    adjacency_matrix: [[usize; MAX_NODES]; MAX_NODES],
    num_nodes: usize,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        assert!(num_nodes > 0 && num_nodes <= MAX_NODES, "Invalid number of nodes");
        let adjacency_matrix = [[INF; MAX_NODES]; MAX_NODES];
        Graph {
            adjacency_matrix,
            num_nodes,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        if from < self.num_nodes && to < self.num_nodes {
            self.adjacency_matrix[from][to] = weight;
        }
    }

    fn dijkstra(&self, start: usize, end: usize) -> Option<usize> {
        if start >= self.num_nodes || end >= self.num_nodes {
            return None;
        }

        let mut distances = vec![INF; self.num_nodes];
        let mut finalized = vec![false; self.num_nodes];
        let mut heap = BinaryHeap::new();

        distances[start] = 0;
        heap.push((0, start)); // (distance, node)

        while let Some((current_distance, current_node)) = heap.pop() {
            if finalized[current_node] {
                continue;
            }
            finalized[current_node] = true;

            for neighbor in 0..self.num_nodes {
                if self.adjacency_matrix[current_node][neighbor] != INF {
                    let new_distance = current_distance + self.adjacency_matrix[current_node][neighbor];
                    if new_distance < distances[neighbor] {
                        distances[neighbor] = new_distance;
                        heap.push((new_distance, neighbor));
                    }
                }
            }
        }

        if distances[end] == INF {
            None
        } else {
            Some(distances[end])
        }
    }

    fn query(&self, start: usize, end: usize, max_length: usize) -> String {
        match self.dijkstra(start, end) {
            Some(length) if length <= max_length => length.to_string(),
            Some(_) => "Path length exceeds the provided value".to_string(),
            None => "Path length exceeds the provided value".to_string(),
        }
    }
}

fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, 10);
    graph.add_edge(0, 2, 5);
    graph.add_edge(1, 2, 2);
    graph.add_edge(1, 3, 1);
    graph.add_edge(2, 1, 3);
    graph.add_edge(2, 3, 9);
    graph.add_edge(2, 4, 2);
    graph.add_edge(3, 4, 4);
    graph.add_edge(4, 3, 6);

    let result = graph.query(0, 4, 15);
    println!("{}", result); // Output the result of the query
}

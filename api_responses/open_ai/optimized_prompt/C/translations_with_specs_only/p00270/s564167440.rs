use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    node: usize,
    weight: usize,
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        let edges = HashMap::with_capacity(num_nodes);
        Graph { edges }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(Edge { node: to, weight });
        self.edges.entry(to).or_insert_with(Vec::new).push(Edge { node: from, weight });
    }

    fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.edges.len()];
        distances[start] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((0, start)); // (distance, node)

        while let Some((current_distance, current_node)) = heap.pop() {
            if current_distance > distances[current_node] {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&current_node) {
                for edge in neighbors {
                    let next = edge.node;
                    let weight = edge.weight;
                    let next_distance = current_distance + weight;

                    if next_distance < distances[next] {
                        distances[next] = next_distance;
                        heap.push((next_distance, next));
                    }
                }
            }
        }

        distances
    }

    fn reachable(&self, start: usize, goal: usize, distances: &[usize]) -> bool {
        distances[goal] != usize::MAX
    }
}

fn main() {
    let num_nodes = 5; // Example number of nodes
    let num_edges = 6; // Example number of edges
    let edges = vec![(0, 1, 10), (0, 2, 5), (1, 2, 2), (1, 3, 1), (2, 1, 3), (2, 3, 9)];
    let start_node = 0; // Example starting node
    let goal_node = 3; // Example goal node
    let queries = vec![(0, 3), (0, 4)]; // Example queries

    let mut graph = Graph::new(num_nodes);
    for (from, to, weight) in edges {
        graph.add_edge(from, to, weight);
    }

    let distances = graph.dijkstra(start_node);

    for (source, destination) in queries {
        if graph.reachable(source, destination, &distances) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

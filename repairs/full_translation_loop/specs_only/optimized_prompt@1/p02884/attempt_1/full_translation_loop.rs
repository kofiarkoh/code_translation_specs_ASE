use std::collections::HashMap;
use std::f64;

struct Graph {
    edges: Vec<(usize, usize)>,
    num_nodes: usize,
}

impl Graph {
    fn new(num_nodes: usize, edges: Vec<(usize, usize)>) -> Self {
        Graph { edges, num_nodes }
    }

    fn average_distance(&self, start_node: usize, remove_edge: Option<(usize, usize)>) -> f64 {
        let mut distances = vec![f64::INFINITY; self.num_nodes];
        distances[start_node] = 0.0;

        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(from, to) in &self.edges {
            adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
        }

        // Remove the specified edge if it exists
        if let Some((from, to)) = remove_edge {
            if let Some(neighbors) = adjacency_list.get_mut(&from) {
                neighbors.retain(|&x| x != to);
            }
        }

        // Perform a simple BFS to calculate distances
        let mut queue = vec![start_node];
        while let Some(node) = queue.pop() {
            if let Some(neighbors) = adjacency_list.get(&node) {
                for &neighbor in neighbors {
                    let new_distance = distances[node] + 1.0; // Assuming each edge has a distance of 1
                    if new_distance < distances[neighbor] {
                        distances[neighbor] = new_distance;
                        queue.push(neighbor);
                    }
                }
            }
        }

        // Calculate the average distance, ignoring unreachable nodes
        let reachable_count = distances.iter().filter(|&&d| d < f64::INFINITY).count();
        let total_distance: f64 = distances.iter().sum();

        if reachable_count == 0 {
            // If no nodes are reachable, return a large constant (as per the specification)
            f64::INFINITY
        } else {
            total_distance / reachable_count as f64
        }
    }

    fn minimum_average_distance(&self, start_node: usize) -> f64 {
        let mut min_average = self.average_distance(start_node, None);

        for &(from, to) in &self.edges {
            let average_with_edge_removed = self.average_distance(start_node, Some((from, to)));
            min_average = min_average.min(average_with_edge_removed);
        }

        // Format the result to ten decimal places
        format!("{:.10}", min_average).parse().unwrap()
    }
}

fn main() {
    let num_nodes = 5;
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    let graph = Graph::new(num_nodes, edges);
    let result = graph.minimum_average_distance(0);
    println!("{:.10}", result);
}

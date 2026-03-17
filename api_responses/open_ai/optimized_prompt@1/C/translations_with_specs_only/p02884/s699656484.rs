use std::collections::HashMap;
use std::f64;

fn main() {
    let (num_nodes, num_edges, edges) = read_input();
    let min_avg_distance = compute_min_avg_distance(num_nodes, num_edges, edges);
    println!("{:.10}", min_avg_distance);
}

fn read_input() -> (usize, usize, Vec<(usize, usize)>) {
    // This function would read the number of nodes, edges, and the edges themselves.
    // For the sake of this example, we will return dummy values.
    // In a real implementation, you would replace this with actual input reading logic.
    (5, 6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)])
}

fn compute_min_avg_distance(num_nodes: usize, num_edges: usize, edges: Vec<(usize, usize)>) -> f64 {
    if num_edges == 0 {
        // If no edges exist, return a large constant as per the specification.
        return f64::INFINITY;
    }

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for (from, to) in edges {
        graph.entry(from).or_insert_with(Vec::new).push(to);
    }

    let mut min_avg_distance = f64::INFINITY;

    // Iterate through each edge to consider its removal
    for (i, (from, to)) in edges.iter().enumerate() {
        let mut distances = vec![f64::INFINITY; num_nodes];
        distances[*from] = 0.0;

        // Perform a modified Dijkstra's algorithm
        let mut queue = vec![*from];
        while let Some(node) = queue.pop() {
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if (node, neighbor) != (*from, *to) { // Skip the removed edge
                        let new_distance = distances[node] + 1.0; // Assuming each edge has a weight of 1
                        if new_distance < distances[neighbor] {
                            distances[neighbor] = new_distance;
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }

        // Calculate the average distance
        let total_distance: f64 = distances.iter().sum();
        let reachable_nodes = distances.iter().filter(|&&d| d < f64::INFINITY).count();
        if reachable_nodes > 0 {
            let avg_distance = total_distance / reachable_nodes as f64;
            min_avg_distance = min_avg_distance.min(avg_distance);
        }
    }

    min_avg_distance
}

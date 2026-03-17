use std::collections::HashMap;
use std::f64;

fn main() {
    let (num_nodes, num_edges, edges) = read_input();
    let min_avg_distance = compute_min_avg_distance(num_nodes, num_edges, &edges);
    println!("{:.10}", min_avg_distance);
}

fn read_input() -> (usize, usize, Vec<(usize, usize)>) {
    (5, 6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)])
}

fn compute_min_avg_distance(num_nodes: usize, num_edges: usize, edges: &Vec<(usize, usize)>) -> f64 {
    if num_edges == 0 {
        return f64::INFINITY;
    }

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for (from, to) in edges {
        graph.entry(*from).or_insert_with(Vec::new).push(*to);
    }

    let mut min_avg_distance = f64::INFINITY;

    for (i, (from, to)) in edges.iter().enumerate() {
        let mut distances = vec![f64::INFINITY; num_nodes];
        distances[*from] = 0.0;

        let mut queue = vec![*from];
        while let Some(node) = queue.pop() {
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if (node, neighbor) != (*from, *to) {
                        let new_distance = distances[node] + 1.0;
                        if new_distance < distances[neighbor] {
                            distances[neighbor] = new_distance;
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }

        let total_distance: f64 = distances.iter().sum();
        let reachable_nodes = distances.iter().filter(|&&d| d < f64::INFINITY).count();
        if reachable_nodes > 0 {
            let avg_distance = total_distance / reachable_nodes as f64;
            min_avg_distance = min_avg_distance.min(avg_distance);
        }
    }

    min_avg_distance
}

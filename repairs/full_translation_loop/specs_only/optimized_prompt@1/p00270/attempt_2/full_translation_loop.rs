use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Edge {
    to: usize,
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
        self.edges.entry(from).or_default().push(Edge { to, weight });
        self.edges.entry(to).or_default().push(Edge { to: from, weight }); // Undirected graph
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
                    let next = edge.to;
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
}

fn can_reach(distances: &[usize], goal: usize) -> bool {
    if goal >= distances.len() {
        return false; // Prevent out-of-bounds access
    }
    distances[goal] != usize::MAX
}

fn main() {
    let num_nodes = 5; // Example input
    let num_edges = 6; // Example input
    let edges = vec![
        (0, 1, 10),
        (0, 2, 5),
        (1, 2, 2),
        (1, 3, 1),
        (2, 1, 3),
        (2, 3, 9),
    ]; // Example edges
    let start_node = 0; // Example start node
    let num_queries = 2; // Example number of queries
    let queries = vec![(0, 3), (1, 4)]; // Example queries

    let mut graph = Graph::new(num_nodes);
    for (from, to, weight) in edges {
        graph.add_edge(from, to, weight);
    }

    let distances = graph.dijkstra(start_node);

    for (_, goal) in queries {
        if can_reach(&distances, goal) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

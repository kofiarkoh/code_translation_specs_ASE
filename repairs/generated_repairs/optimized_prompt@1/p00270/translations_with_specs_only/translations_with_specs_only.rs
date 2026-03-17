use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Edge {
    to: usize,
    weight: usize,
}

#[derive(Debug)]
struct Graph {
    nodes: usize,
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Graph {
            nodes,
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(Edge { to, weight });
        self.edges.entry(to).or_insert_with(Vec::new).push(Edge { to: from, weight }); // Undirected graph
    }

    fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.nodes];
        distances[start] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((0, start)); // (distance, node)

        while let Some((current_distance, current_node)) = heap.pop() {
            if current_distance > distances[current_node] {
                continue; // Skip if we found a better path
            }

            if let Some(neighbors) = self.edges.get(&current_node) {
                for edge in neighbors {
                    let next_distance = current_distance + edge.weight;

                    if next_distance < distances[edge.to] {
                        distances[edge.to] = next_distance;
                        heap.push((next_distance, edge.to));
                    }
                }
            }
        }

        distances
    }

    fn can_reach(&self, start: usize, goal: usize) -> bool {
        let distances = self.dijkstra(start);
        distances[goal] != usize::MAX // If the distance is still MAX, it means it's unreachable
    }
}

fn main() {
    let nodes = 5; // Example number of nodes
    let edges = 6; // Example number of edges
    let mut graph = Graph::new(nodes);

    // Example edges (from, to, weight)
    graph.add_edge(1, 2, 4);
    graph.add_edge(1, 3, 2);
    graph.add_edge(2, 3, 1);
    graph.add_edge(2, 4, 5);
    graph.add_edge(3, 4, 8);
    graph.add_edge(3, 5, 10);

    // Example queries
    let queries = vec![(1, 4), (2, 5)];
    for (start, goal) in queries {
        if graph.can_reach(start, goal) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

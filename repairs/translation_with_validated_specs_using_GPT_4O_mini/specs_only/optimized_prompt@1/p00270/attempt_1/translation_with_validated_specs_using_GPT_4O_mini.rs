use std::collections::{BinaryHeap, HashMap};
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
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.edges.entry(from).or_default().push(Edge { node: to, weight });
        self.edges.entry(to).or_default().push(Edge { node: from, weight });
    }

    fn dijkstra(&self, start: usize) -> Vec<Option<usize>> {
        let mut distances = vec![None; self.edges.len()];
        distances[start] = Some(0);
        let mut heap = BinaryHeap::new();
        heap.push(State { cost: 0, position: start });

        while let Some(State { cost, position }) = heap.pop() {
            if let Some(&Some(current_distance)) = distances.get(position) {
                if cost > current_distance {
                    continue; // Skip if we have already found a better path
                }
            }

            if let Some(neighbors) = self.edges.get(&position) {
                for edge in neighbors {
                    let next = edge.node;
                    let next_cost = cost + edge.weight;

                    if distances[next].is_none() || next_cost < distances[next].unwrap() {
                        distances[next] = Some(next_cost);
                        heap.push(State { cost: next_cost, position: next });
                    }
                }
            }
        }

        distances
    }
}

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// Implementing Ord and PartialOrd for State to use in BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse ordering for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let num_nodes = 5; // Example number of nodes
    let num_edges = 6; // Example number of edges
    let edges = vec![
        (0, 1, 10),
        (0, 2, 5),
        (1, 2, 2),
        (1, 3, 1),
        (2, 1, 3),
        (2, 3, 9),
        (2, 4, 2),
        (3, 4, 4),
    ];
    let start_node = 0; // Example starting node
    let queries = vec![(3, 4), (0, 1), (4, 0)]; // Example queries

    let mut graph = Graph::new();
    for (from, to, weight) in edges {
        graph.add_edge(from, to, weight);
    }

    let distances = graph.dijkstra(start_node);

    for (query_start, query_end) in queries {
        if let Some(distance) = distances[query_end] {
            if distance < usize::MAX {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}

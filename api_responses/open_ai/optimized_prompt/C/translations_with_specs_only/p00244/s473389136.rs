use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    to: usize,
    cost: i32,
}

#[derive(Debug)]
struct Node {
    index: usize,
    cost: i32,
    free_travels: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap based on cost
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.index == other.index && self.free_travels == other.free_travels
    }
}

impl Eq for Node {}

pub struct Graph {
    adjacency_list: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new(num_nodes: usize) -> Self {
        let mut adjacency_list = HashMap::new();
        for i in 0..num_nodes {
            adjacency_list.insert(i, Vec::new());
        }
        Graph { adjacency_list }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: i32) {
        if let Some(edges) = self.adjacency_list.get_mut(&from) {
            edges.push(Edge { to, cost });
        }
    }

    pub fn min_cost(&self, start: usize, goal: usize, free_travels: i32) -> i32 {
        let mut min_costs = vec![i32::MAX; self.adjacency_list.len()];
        let mut heap = BinaryHeap::new();
        
        // Start with the initial node
        min_costs[start] = 0;
        heap.push(Node { index: start, cost: 0, free_travels });

        while let Some(Node { index, cost, free_travels }) = heap.pop() {
            // If we reach the goal, return the cost
            if index == goal {
                return cost;
            }

            // If the cost is greater than the recorded minimum, skip
            if cost > min_costs[index] {
                continue;
            }

            // Explore neighbors
            if let Some(edges) = self.adjacency_list.get(&index) {
                for edge in edges {
                    let next_cost = cost + edge.cost;

                    // Check if we can use a free travel
                    if free_travels > 0 {
                        let free_travel_cost = cost; // No cost for free travel
                        if free_travel_cost < min_costs[edge.to] {
                            min_costs[edge.to] = free_travel_cost;
                            heap.push(Node { index: edge.to, cost: free_travel_cost, free_travels: free_travels - 1 });
                        }
                    }

                    // Normal travel
                    if next_cost < min_costs[edge.to] {
                        min_costs[edge.to] = next_cost;
                        heap.push(Node { index: edge.to, cost: next_cost, free_travels });
                    }
                }
            }
        }

        // If the goal is unreachable, return a specific value (e.g., -1)
        -1 // This value indicates that the goal is unreachable
    }
}

// Example usage
fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, 10);
    graph.add_edge(1, 2, 5);
    graph.add_edge(0, 2, 15);
    graph.add_edge(2, 3, 10);
    graph.add_edge(3, 4, 5);

    let min_cost = graph.min_cost(0, 4, 1);
    println!("Minimum cost from node 0 to node 4: {}", min_cost);
}

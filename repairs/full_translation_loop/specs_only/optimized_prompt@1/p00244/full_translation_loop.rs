use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    target: usize,
    cost: i32,
}

#[derive(Debug)]
struct Node {
    index: usize,
    cost: i32,
    free_transitions: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap based on cost, so we reverse the ordering
        other.cost.cmp(&self.cost)
            .then_with(|| self.free_transitions.cmp(&other.free_transitions))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.free_transitions == other.free_transitions
    }
}

impl Eq for Node {}

struct Graph {
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: i32) {
        self.edges.entry(from).or_default().push(Edge { target: to, cost });
        self.edges.entry(to).or_default().push(Edge { target: from, cost });
    }

    fn min_cost_with_free_transitions(&self, start: usize, goal: usize, free_transitions: i32) -> i32 {
        let mut min_cost = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        heap.push(Node { index: start, cost: 0, free_transitions });
        min_cost.insert((start, free_transitions), 0);

        while let Some(Node { index, cost, free_transitions }) = heap.pop() {
            if index == goal {
                return cost; // Found the goal
            }

            if let Some(edges) = self.edges.get(&index) {
                for edge in edges {
                    let next_cost = cost + edge.cost;

                    // Transition without using a free transition
                    if next_cost < *min_cost.get(&(edge.target, free_transitions)).unwrap_or(&i32::MAX) {
                        min_cost.insert((edge.target, free_transitions), next_cost);
                        heap.push(Node { index: edge.target, cost: next_cost, free_transitions });
                    }

                    // Transition using a free transition if available
                    if free_transitions > 0 {
                        if cost < *min_cost.get(&(edge.target, free_transitions - 1)).unwrap_or(&i32::MAX) {
                            min_cost.insert((edge.target, free_transitions - 1), cost);
                            heap.push(Node { index: edge.target, cost, free_transitions: free_transitions - 1 });
                        }
                    }
                }
            }
        }

        // If we reach here, the goal is unreachable
        -1 // Return -1 to indicate that the goal is unreachable, as per the specification
    }
}

fn main() {
    // Example usage:
    let mut graph = Graph::new();
    graph.add_edge(1, 2, 10);
    graph.add_edge(2, 3, 10);
    graph.add_edge(3, 4, 10);
    graph.add_edge(1, 3, 30);
    
    let min_cost = graph.min_cost_with_free_transitions(1, 4, 0);
    println!("{}", min_cost); // Should print -1 for unreachable case
}

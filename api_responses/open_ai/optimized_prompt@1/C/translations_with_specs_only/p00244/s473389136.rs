use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    target: usize,
    cost: i32,
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

    fn add_edge(&mut self, from: usize, to: usize, cost: i32) {
        self.edges.entry(from).or_default().push(Edge { target: to, cost });
        self.edges.entry(to).or_default().push(Edge { target: from, cost });
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: i32,
    node: usize,
    free_transitions: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_cost_with_free_transitions(
    graph: &Graph,
    start: usize,
    goal: usize,
    free_transitions: usize,
) -> i32 {
    let mut min_cost = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, node: start, free_transitions });

    while let Some(State { cost, node, free_transitions }) = heap.pop() {
        if node == goal {
            return cost; // Found the goal
        }

        if let Some(&prev_cost) = min_cost.get(&(node, free_transitions)) {
            if cost >= prev_cost {
                continue; // Already found a better way
            }
        }

        min_cost.insert((node, free_transitions), cost);

        if let Some(edges) = graph.edges.get(&node) {
            for edge in edges {
                // Transition without using a free transition
                let new_cost = cost + edge.cost;
                heap.push(State { cost: new_cost, node: edge.target, free_transitions });

                // Transition using a free transition if available
                if free_transitions > 0 {
                    heap.push(State { cost, node: edge.target, free_transitions: free_transitions - 1 });
                }
            }
        }
    }

    -1 // Indicate that the goal is unreachable
}

fn main() {
    let num_nodes = 5; // Example number of nodes
    let num_edges = 6; // Example number of edges
    let mut graph = Graph::new(num_nodes);

    // Example edges (from, to, cost)
    graph.add_edge(0, 1, 10);
    graph.add_edge(0, 2, 5);
    graph.add_edge(1, 2, 2);
    graph.add_edge(1, 3, 1);
    graph.add_edge(2, 3, 9);
    graph.add_edge(3, 4, 4);

    let start = 0;
    let goal = 4;
    let free_transitions = 1;

    let min_cost = min_cost_with_free_transitions(&graph, start, goal, free_transitions);
    println!("Minimum cost to reach goal: {}", min_cost);
}

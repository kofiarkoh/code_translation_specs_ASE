use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
struct Edge {
    cost: usize,
    node: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

fn min_cost_path(size: usize, m: usize, edges: &[(usize, usize, usize)], start: usize, goal: usize, free_moves: usize) -> usize {
    // Create adjacency list
    let mut graph: HashMap<usize, Vec<Edge>> = HashMap::new();
    for &(a, b, c) in edges {
        graph.entry(a).or_insert_with(Vec::new).push(Edge { cost: c, node: b });
        graph.entry(b).or_insert_with(Vec::new).push(Edge { cost: c, node: a });
    }

    // Priority queue for Dijkstra's algorithm
    let mut heap = BinaryHeap::new();
    let mut costs = vec![usize::MAX; size];
    let mut free_moves_used = vec![0; size];

    // Start from the initial node
    costs[start] = 0;
    heap.push(Edge { cost: 0, node: start });

    while let Some(Edge { cost, node }) = heap.pop() {
        if node == goal {
            return cost; // Return the cost when we reach the goal
        }

        // If the cost is greater than the recorded cost, skip
        if cost > costs[node] {
            continue;
        }

        // Explore neighbors
        if let Some(neighbors) = graph.get(&node) {
            for &Edge { cost: edge_cost, node: neighbor } in neighbors {
                let new_cost = cost + edge_cost;

                // Check if we can reach the neighbor without using a free move
                if new_cost < costs[neighbor] {
                    costs[neighbor] = new_cost;
                    heap.push(Edge { cost: new_cost, node: neighbor });
                }

                // Check if we can reach the neighbor using a free move
                if free_moves_used[node] < free_moves {
                    let free_move_cost = cost; // No additional cost for free move
                    if free_move_cost < costs[neighbor] {
                        costs[neighbor] = free_move_cost;
                        free_moves_used[neighbor] = free_moves_used[node] + 1;
                        heap.push(Edge { cost: free_move_cost, node: neighbor });
                    }
                }
            }
        }
    }

    // If we reach here, no path was found
    usize::MAX
}

fn main() {
    let size = 5; // Example size
    let m = 6; // Example number of edges
    let edges = vec![
        (0, 1, 10),
        (0, 2, 5),
        (1, 2, 2),
        (1, 3, 1),
        (2, 3, 9),
        (3, 4, 4),
    ];
    let start = 0; // Starting node
    let goal = 4; // Goal node
    let free_moves = 1; // Number of free moves

    let min_cost = min_cost_path(size, m, &edges, start, goal, free_moves);
    
    // Output the result
    if min_cost == usize::MAX {
        println!("No path exists");
    } else {
        println!("Minimum cost: {}", min_cost);
    }
}

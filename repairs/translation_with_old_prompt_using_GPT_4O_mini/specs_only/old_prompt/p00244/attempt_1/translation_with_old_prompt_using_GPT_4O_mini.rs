use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
struct Edge {
    node: usize,
    cost: i32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    cost: i32,
    node: usize,
    free_moves: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node && self.free_moves == other.free_moves
    }
}

impl Eq for State {}

fn dijkstra_with_free_moves(size: usize, edges: Vec<(usize, usize, i32)>, start: usize, goal: usize, free_moves: i32) -> i32 {
    let mut graph: HashMap<usize, Vec<Edge>> = HashMap::new();
    
    // Build the graph
    for (a, b, c) in edges {
        graph.entry(a).or_insert_with(Vec::new).push(Edge { node: b, cost: c });
        graph.entry(b).or_insert_with(Vec::new).push(Edge { node: a, cost: c });
    }

    let mut min_cost = vec![vec![i32::MAX; (free_moves + 1) as usize]; size];
    let mut heap = BinaryHeap::new();

    // Start from the initial node with 0 cost and available free moves
    min_cost[start][free_moves as usize] = 0;
    heap.push(State { cost: 0, node: start, free_moves });

    while let Some(State { cost, node, free_moves }) = heap.pop() {
        // If we reached the goal, return the cost
        if node == goal {
            return cost;
        }

        // If the cost is greater than the recorded minimum, skip
        if cost > min_cost[node][free_moves as usize] {
            continue;
        }

        // Explore neighbors
        if let Some(edges) = graph.get(&node) {
            for Edge { node: neighbor, cost: edge_cost } in edges {
                let new_cost = cost + edge_cost;

                // If we can move to the neighbor without using a free move
                if new_cost < min_cost[*neighbor][free_moves as usize] {
                    min_cost[*neighbor][free_moves as usize] = new_cost;
                    heap.push(State { cost: new_cost, node: *neighbor, free_moves });
                }

                // If we can use a free move to the neighbor
                if free_moves > 0 {
                    if cost < min_cost[*neighbor][(free_moves - 1) as usize] {
                        min_cost[*neighbor][(free_moves - 1) as usize] = cost;
                        heap.push(State { cost, node: *neighbor, free_moves: free_moves - 1 });
                    }
                }
            }
        }
    }

    // If we reach here, no path was found
    -1 // Indicate no path exists
}

fn main() {
    let size = 5; // Example size
    let edges = vec![(0, 1, 10), (1, 2, 10), (0, 2, 15), (2, 3, 10), (3, 4, 5)]; // Example edges
    let start = 0; // Starting node
    let goal = 4; // Goal node
    let free_moves = 1; // Number of free moves

    let min_cost = dijkstra_with_free_moves(size, edges, start, goal, free_moves);
    println!("{}", min_cost);
}

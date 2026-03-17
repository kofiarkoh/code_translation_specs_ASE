use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    to: usize,
    capacity: i32,
    cost: i32,
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

    fn add_edge(&mut self, from: usize, to: usize, capacity: i32, cost: i32) {
        self.edges.entry(from).or_default().push(Edge { to, capacity, cost });
    }
}

#[derive(Debug)]
struct State {
    cost: i32,
    node: usize,
    flow: i32,
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

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Eq for State {}

fn min_cost_flow(graph: &Graph, source: usize, sink: usize, flow_limit: i32) -> i32 {
    let mut total_cost = 0;
    let mut flow = 0;

    while flow < flow_limit {
        let mut dist = vec![i32::MAX; graph.edges.len()];
        let mut parent = vec![-1; graph.edges.len()];
        let mut heap = BinaryHeap::new();
        dist[source] = 0;
        heap.push(State { cost: 0, node: source, flow: 0 });

        while let Some(State { cost, node, flow: _ }) = heap.pop() {
            if cost > dist[node] {
                continue;
            }

            if let Some(edges) = graph.edges.get(&node) {
                for edge in edges {
                    if edge.capacity > 0 {
                        let new_cost = cost + edge.cost;
                        if new_cost < dist[edge.to] {
                            dist[edge.to] = new_cost;
                            parent[edge.to] = node as i32;
                            heap.push(State { cost: new_cost, node: edge.to, flow: 0 });
                        }
                    }
                }
            }
        }

        if dist[sink] == i32::MAX {
            return -1; // Flow cannot be achieved
        }

        let mut current_flow = flow_limit - flow;
        let mut node = sink;

        while node != source as i32 {
            let prev = parent[node as usize];
            if prev == -1 {
                break; // No path found
            }

            if let Some(edges) = graph.edges.get(&(prev as usize)) {
                for edge in edges {
                    if edge.to == node as usize {
                        current_flow = current_flow.min(edge.capacity);
                    }
                }
            }
            node = prev;
        }

        flow += current_flow;
        total_cost += current_flow * dist[sink];

        // Update capacities
        node = sink;
        while node != source as i32 {
            let prev = parent[node as usize];
            if prev == -1 {
                break; // No path found
            }

            if let Some(edges) = graph.edges.get_mut(&(prev as usize)) {
                for edge in edges {
                    if edge.to == node as usize {
                        edge.capacity -= current_flow;
                    }
                }
            }
            node = prev;
        }
    }

    total_cost
}

fn main() {
    let n: usize = 5; // Example number of nodes
    let m: usize = 6; // Example number of edges
    let b: i32 = 10; // Example constant affecting costs
    let source_nodes = vec![(0, 0), (1, 1)]; // Example source coordinates
    let sink_nodes = vec![(2, 2), (3, 3)]; // Example sink coordinates
    let capacities = vec![10, 20]; // Example capacities for sinks
    let flow_limits = vec![5, 15]; // Example flow limits for sinks

    let mut graph = Graph::new();

    // Example edges (from, to, capacity, cost)
    graph.add_edge(0, 2, 10, 1);
    graph.add_edge(1, 3, 20, 2);
    graph.add_edge(2, 4, 10, 1);
    graph.add_edge(3, 4, 5, 3);

    let min_cost = min_cost_flow(&graph, 0, 4, 15);
    println!("{}", min_cost); // Output the minimum cost
}

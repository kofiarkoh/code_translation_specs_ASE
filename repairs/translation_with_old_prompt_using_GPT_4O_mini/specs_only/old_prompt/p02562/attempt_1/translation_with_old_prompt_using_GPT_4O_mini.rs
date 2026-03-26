use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INF: i64 = std::i64::MAX;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    capacity: i64,
    cost: i64,
    flow: i64,
}

struct FlowNetwork {
    adj: Vec<Vec<usize>>,
    edges: Vec<Edge>,
    costs: Vec<Vec<i64>>,
    n: usize,
}

impl FlowNetwork {
    fn new(n: usize, costs: Vec<Vec<i64>>) -> Self {
        let mut adj = vec![Vec::new(); n];
        let mut edges = Vec::new();
        
        for i in 0..n {
            for j in 0..n {
                if costs[i][j] < 0 {
                    // Assuming negative costs are invalid; this should be handled as per the specification.
                    panic!("Invalid cost in the cost matrix");
                }
                if costs[i][j] > 0 {
                    let edge = Edge {
                        to: j,
                        capacity: INF, // Assuming maximum capacity for simplicity
                        cost: costs[i][j],
                        flow: 0,
                    };
                    edges.push(edge);
                    adj[i].push(edges.len() - 1); // Store the index of the edge
                }
            }
        }
        
        FlowNetwork { adj, edges, costs, n }
    }

    fn successive_shortest_path(&mut self) -> i64 {
        let mut total_cost = 0;
        let mut flow = vec![0; self.edges.len()];

        loop {
            let (dist, prev_edge) = self.shortest_path();
            if dist.is_empty() {
                break; // No more augmenting path
            }

            let mut increment = INF;
            let mut v = self.n - 1; // Assuming the sink is the last node
            while v != 0 {
                let edge_index = prev_edge[v];
                let edge = &self.edges[edge_index];
                increment = increment.min(edge.capacity - edge.flow);
                v = edge.to;
            }

            // Update flow and costs
            v = self.n - 1;
            while v != 0 {
                let edge_index = prev_edge[v];
                self.edges[edge_index].flow += increment;
                total_cost += self.edges[edge_index].cost * increment;
                v = self.edges[edge_index].to;
            }
        }

        total_cost
    }

    fn shortest_path(&self) -> (Vec<i64>, Vec<usize>) {
        let mut dist = vec![INF; self.n];
        let mut prev_edge = vec![usize::MAX; self.n];
        let mut heap = BinaryHeap::new();
        
        dist[0] = 0; // Start from the source
        heap.push((0, 0)); // (cost, node)

        while let Some((cost, u)) = heap.pop() {
            if cost > dist[u] {
                continue; // Skip if we found a better path
            }

            for &edge_index in &self.adj[u] {
                let edge = &self.edges[edge_index];
                let new_cost = cost + edge.cost;

                if edge.flow < edge.capacity && new_cost < dist[edge.to] {
                    dist[edge.to] = new_cost;
                    prev_edge[edge.to] = edge_index;
                    heap.push((new_cost, edge.to));
                }
            }
        }

        (dist, prev_edge)
    }
}

fn main() {
    let n = 5; // Example number of nodes
    let k = 10; // Example maximum capacity
    let costs = vec![
        vec![0, 1, 2, 0, 0],
        vec![0, 0, 0, 3, 0],
        vec![0, 0, 0, 0, 4],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let mut network = FlowNetwork::new(n, costs);
    let total_cost = network.successive_shortest_path();

    println!("Total cost of flow: {}", total_cost);
    // Output flow configuration as a matrix of 'X' for active flows
    for i in 0..n {
        for j in 0..n {
            if network.edges.iter().any(|e| e.to == j && e.flow > 0) {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

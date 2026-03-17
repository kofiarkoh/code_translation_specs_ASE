use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
struct Edge {
    to: usize,
    capacity: f64,
    cost: f64,
    flow: f64,
}

#[derive(Debug)]
struct Graph {
    nodes: usize,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Graph {
            nodes,
            edges: vec![Vec::new(); nodes],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: f64, cost: f64) {
        self.edges[from].push(Edge {
            to,
            capacity,
            cost,
            flow: 0.0,
        });
    }

    fn min_cost_flow(&mut self, source: usize, target: usize, flow: f64) -> Result<f64, &'static str> {
        let mut total_cost = 0.0;
        let mut remaining_flow = flow;

        while remaining_flow > 0.0 {
            let (cost, path_flow) = self.bellman_ford(source, target, remaining_flow)?;
            if path_flow == 0.0 {
                return Err("impossible");
            }
            total_cost += cost;
            remaining_flow -= path_flow;
        }

        Ok(total_cost)
    }

    fn bellman_ford(&self, source: usize, target: usize, flow: f64) -> Result<(f64, f64), &'static str> {
        let mut dist = vec![f64::INFINITY; self.nodes];
        let mut parent = vec![None; self.nodes];
        let mut in_queue = vec![false; self.nodes];
        let mut queue = VecDeque::new();

        dist[source] = 0.0;
        queue.push_back(source);
        in_queue[source] = true;

        while let Some(u) = queue.pop_front() {
            in_queue[u] = false;

            for edge in &self.edges[u] {
                if edge.flow < edge.capacity {
                    let new_cost = dist[u] + edge.cost;
                    if new_cost < dist[edge.to] {
                        dist[edge.to] = new_cost;
                        parent[edge.to] = Some((u, edge));
                        if !in_queue[edge.to] {
                            queue.push_back(edge.to);
                            in_queue[edge.to] = true;
                        }
                    }
                }
            }
        }

        if dist[target] == f64::INFINITY {
            return Err("impossible");
        }

        let mut path_flow = flow;
        let mut u = target;

        while let Some((prev, edge)) = parent[u].take() {
            path_flow = path_flow.min(edge.capacity - edge.flow);
            u = prev;
        }

        u = target;
        while let Some((prev, edge)) = parent[u].take() {
            // Update flow along the path
            edge.flow += path_flow;
            u = prev;
        }

        Ok((dist[target] * path_flow, path_flow))
    }
}

fn main() {
    let t: usize = read_input(); // Read number of test cases
    for _ in 0..t {
        let (n, s, t, f) = read_graph_parameters(); // Read n, s, t, F
        let mut graph = Graph::new(n);

        for _ in 0..n {
            let (from, to, capacity, cost) = read_edge_parameters(); // Read edge parameters
            graph.add_edge(from, to, capacity, cost);
        }

        match graph.min_cost_flow(s, t, f) {
            Ok(cost) => println!("{:.10}", cost),
            Err(_) => println!("impossible"),
        }
    }
}

// Placeholder functions for reading input
fn read_input() -> usize {
    // Implement input reading logic
    0
}

fn read_graph_parameters() -> (usize, usize, usize, f64) {
    // Implement input reading logic
    (0, 0, 0, 0.0)
}

fn read_edge_parameters() -> (usize, usize, f64, f64) {
    // Implement input reading logic
    (0, 0, 0.0, 0.0)
}

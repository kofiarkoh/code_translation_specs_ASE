use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Taxi {
    cost: usize, // Cost of taxi service in town
    limit: usize, // Maximum number of rides available in town
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Status {
    town: usize, // Town identifier
    remaining_rides: usize, // Remaining rides
    cost: usize, // Cost to reach this town
}

// Implementing Ord for Status to use it in a priority queue
impl Ord for Status {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap, so we reverse the ordering
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read N and K
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let k: usize = params.next().unwrap().parse().unwrap();

    // Read taxi data
    let mut taxis = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut taxi_data = line.split_whitespace();
        let cost: usize = taxi_data.next().unwrap().parse().unwrap();
        let limit: usize = taxi_data.next().unwrap().parse().unwrap();
        taxis.push(Taxi { cost, limit });
    }

    // Read connections
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut connection_data = line.split_whitespace();
        let from: usize = connection_data.next().unwrap().parse().unwrap();
        let to: usize = connection_data.next().unwrap().parse().unwrap();
        let cost: usize = connection_data.next().unwrap().parse().unwrap();
        graph[from].push((to, cost));
        graph[to].push((from, cost)); // Assuming undirected graph
    }

    // Dijkstra's algorithm to find the minimum cost
    let mut min_cost = vec![vec![usize::MAX; n]; n + 1]; // min_cost[remaining_rides][town]
    let mut heap = BinaryHeap::new();

    // Initialize the heap with starting town (0) and all available rides
    for rides in 0..=taxis[0].limit {
        min_cost[rides][0] = 0;
        heap.push(Status { town: 0, remaining_rides: rides, cost: 0 });
    }

    while let Some(Status { town, remaining_rides, cost }) = heap.pop() {
        // If we reach the last town (n-1), print the cost
        if town == n - 1 {
            println!("{}", cost);
            return;
        }

        // Explore neighbors
        for &(neighbor, edge_cost) in &graph[town] {
            if remaining_rides < taxis[town].limit {
                let new_cost = cost + edge_cost + taxis[neighbor].cost;
                if new_cost < min_cost[remaining_rides + 1][neighbor] {
                    min_cost[remaining_rides + 1][neighbor] = new_cost;
                    heap.push(Status { town: neighbor, remaining_rides: remaining_rides + 1, cost: new_cost });
                }
            }
        }
    }

    // If no valid path exists, no output is specified
}

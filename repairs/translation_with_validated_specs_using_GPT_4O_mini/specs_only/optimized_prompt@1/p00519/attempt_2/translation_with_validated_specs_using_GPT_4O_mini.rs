use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Town {
    cost: i32,
    limit: i32,
}

#[derive(Debug)]
struct Edge {
    to: usize,
    cost: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    town: usize,
    remaining_rides: i32,
    total_cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost.cmp(&self.total_cost) // Min-heap based on total cost
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    
    let n: usize = params.next().unwrap().parse().unwrap();
    let k: usize = params.next().unwrap().parse().unwrap();

    if n == 0 || (k > 0 && k > (n * (n - 1)) / 2) {
        return; // Invalid input, terminate without output
    }

    let mut towns = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut details = line.split_whitespace();
        let cost: i32 = details.next().unwrap().parse().unwrap();
        let limit: i32 = details.next().unwrap().parse().unwrap();
        towns.push(Town { cost, limit });
    }

    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); n]; // Fixed: Initialize graph correctly
    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut connection = line.split_whitespace();
        let from: usize = connection.next().unwrap().parse().unwrap();
        let to: usize = connection.next().unwrap().parse().unwrap();
        graph[from].push(Edge { to, cost: towns[to].cost });
        graph[to].push(Edge { to: from, cost: towns[from].cost });
    }

    let mut min_cost = vec![vec![i32::MAX; n + 1]; n];
    let mut heap = BinaryHeap::new();
    min_cost[0][0] = 0;
    heap.push(State { town: 0, remaining_rides: 0, total_cost: 0 });

    while let Some(State { town, remaining_rides, total_cost }) = heap.pop() {
        if town == n - 1 {
            println!("{}", total_cost);
            return; // Output the minimum cost to reach the last town
        }

        if total_cost > min_cost[town][remaining_rides as usize] {
            continue; // Skip if we already found a better cost
        }

        for edge in &graph[town] {
            let next_cost = total_cost + edge.cost;
            if next_cost < min_cost[edge.to][remaining_rides as usize] {
                min_cost[edge.to][remaining_rides as usize] = next_cost;
                heap.push(State { town: edge.to, remaining_rides, total_cost: next_cost });
            }
        }

        if remaining_rides < towns[town].limit {
            let next_rides = remaining_rides + 1;
            let next_cost = total_cost + towns[town].cost;
            if next_cost < min_cost[town][next_rides as usize] {
                min_cost[town][next_rides as usize] = next_cost;
                heap.push(State { town, remaining_rides: next_rides, total_cost: next_cost });
            }
        }
    }
}

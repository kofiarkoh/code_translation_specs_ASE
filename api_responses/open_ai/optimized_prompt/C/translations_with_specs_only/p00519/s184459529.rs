use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Town {
    fare: usize,
    max_rides: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    town: usize,
    rides: usize,
}

// Implementing Ord and PartialOrd for State to use it in a BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap, so we reverse the ordering
        other.cost.cmp(&self.cost)
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

    // Read number of towns and connections
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let num_towns: usize = parts.next().unwrap().parse().unwrap();
    let num_connections: usize = parts.next().unwrap().parse().unwrap();

    // Read town data
    let mut towns = Vec::with_capacity(num_towns);
    for _ in 0..num_towns {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let fare: usize = parts.next().unwrap().parse().unwrap();
        let max_rides: usize = parts.next().unwrap().parse().unwrap();
        towns.push(Town { fare, max_rides });
    }

    // Read connections
    let mut graph = vec![Vec::new(); num_towns];
    for _ in 0..num_connections {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let town_a: usize = parts.next().unwrap().parse().unwrap();
        let town_b: usize = parts.next().unwrap().parse().unwrap();
        graph[town_a].push(town_b);
        graph[town_b].push(town_a);
    }

    // Dijkstra's algorithm to find the minimum cost
    let mut min_cost = vec![vec![usize::MAX; num_towns]; num_towns];
    let mut heap = BinaryHeap::new();
    min_cost[0][0] = 0; // Starting at town 0 with 0 rides
    heap.push(State { cost: 0, town: 0, rides: 0 });

    while let Some(State { cost, town, rides }) = heap.pop() {
        // If we reached the last town, print the cost and return
        if town == num_towns - 1 {
            println!("{}", cost);
            return;
        }

        // If the current cost is greater than the recorded minimum, skip
        if cost > min_cost[rides][town] {
            continue;
        }

        // Explore neighbors
        for &neighbor in &graph[town] {
            if rides < towns[town].max_rides {
                let new_cost = cost + towns[neighbor].fare;
                if new_cost < min_cost[rides + 1][neighbor] {
                    min_cost[rides + 1][neighbor] = new_cost;
                    heap.push(State { cost: new_cost, town: neighbor, rides: rides + 1 });
                }
            }
        }
    }
}

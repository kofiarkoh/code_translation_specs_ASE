use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Edge {
    to: usize,
    distance: usize,
}

#[derive(Debug)]
struct Node {
    city: usize,
    distance: usize,
    fuel: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse ordering for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.fuel == other.fuel
    }
}

impl Eq for Node {}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of cities (N), LPG stations (M), and fuel capacity (cap)
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let n = params[0]; // Number of cities
    let m = params[1]; // Number of LPG stations
    let cap = params[2]; // Fuel capacity

    // Initialize graph
    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); n];

    // Read road distances
    for _ in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from: usize = parts[0].parse().unwrap();
        let to: usize = parts[1].parse().unwrap();
        let distance: usize = parts[2].parse().unwrap();
        graph[from].push(Edge { to, distance });
        graph[to].push(Edge { to: from, distance }); // Undirected graph
    }

    // Read LPG stations
    let mut lpg_stations = vec![];
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let station: usize = line.parse().unwrap();
        lpg_stations.push(station);
    }

    // Dijkstra's algorithm with fuel constraints
    let mut distances = vec![vec![usize::MAX; cap + 1]; n];
    let mut heap = BinaryHeap::new();
    distances[0][cap] = 0; // Starting city with full fuel
    heap.push(Node { city: 0, distance: 0, fuel: cap });

    while let Some(Node { city, distance, fuel }) = heap.pop() {
        if distance > distances[city][fuel] {
            continue; // Skip if we found a better path
        }

        // Check if we can refuel at this city
        if lpg_stations.contains(&(city as usize)) {
            if fuel < cap {
                let new_fuel = cap;
                if distance < distances[city][new_fuel] {
                    distances[city][new_fuel] = distance;
                    heap.push(Node { city, distance, fuel: new_fuel });
                }
            }
        }

        // Explore neighbors
        for edge in &graph[city] {
            if fuel >= edge.distance {
                let new_distance = distance + edge.distance;
                let new_fuel = fuel - edge.distance;
                if new_distance < distances[edge.to][new_fuel] {
                    distances[edge.to][new_fuel] = new_distance;
                    heap.push(Node { city: edge.to, distance: new_distance, fuel: new_fuel });
                }
            }
        }
    }

    // Find the minimum distance to the destination city (n-1)
    let min_distance = distances[n - 1].iter().cloned().min().unwrap_or(usize::MAX);
    if min_distance == usize::MAX {
        println!("-1");
    } else {
        println!("{}", min_distance);
    }
}

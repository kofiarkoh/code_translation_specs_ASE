use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug)]
struct Town {
    fare: usize,
    max_rides: usize,
}

#[derive(Debug)]
struct Connection {
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct State {
    town: usize,
    cost: usize,
    rides_left: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Min-heap based on cost
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.rides_left == other.rides_left
    }
}

impl Eq for State {}

fn min_cost_to_destination(
    towns: Vec<Town>,
    connections: Vec<Connection>,
    start: usize,
    destination: usize,
) -> Option<usize> {
    let n = towns.len();
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    
    // Build the graph from connections
    for conn in connections {
        graph.entry(conn.from).or_default().push(conn.to);
        graph.entry(conn.to).or_default().push(conn.from);
    }

    let mut min_cost = vec![vec![usize::MAX; 501]; n]; // 501 for max rides
    let mut heap = BinaryHeap::new();
    
    // Initialize the starting town
    min_cost[start][0] = 0;
    heap.push(State { town: start, cost: 0, rides_left: 0 });

    while let Some(State { town, cost, rides_left }) = heap.pop() {
        if town == destination {
            return Some(cost);
        }

        if cost > min_cost[town][rides_left] {
            continue; // Skip if we already found a better cost
        }

        // Explore neighbors
        for &neighbor in graph.get(&town).unwrap_or(&vec![]) {
            if rides_left < towns[neighbor].max_rides {
                let new_cost = cost + towns[neighbor].fare;
                let new_rides = rides_left + 1;

                if new_cost < min_cost[neighbor][new_rides] {
                    min_cost[neighbor][new_rides] = new_cost;
                    heap.push(State { town: neighbor, cost: new_cost, rides_left: new_rides });
                }
            }
        }
    }

    None // If no path to destination was found
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let first_line: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (num_towns, num_connections) = (first_line[0], first_line[1]);

    let mut towns = Vec::with_capacity(num_towns);
    for _ in 0..num_towns {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let town_info: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        towns.push(Town { fare: town_info[0], max_rides: town_info[1] });
    }

    let mut connections = Vec::with_capacity(num_connections);
    for _ in 0..num_connections {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let conn_info: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        connections.push(Connection { from: conn_info[0] - 1, to: conn_info[1] - 1 }); // Adjust for 0-based index
    }

    let start = 0; // Starting town (0-based)
    let destination = num_towns - 1; // Destination town (last town)

    if let Some(cost) = min_cost_to_destination(towns, connections, start, destination) {
        println!("{}", cost); // Output the minimum cost directly
    } else {
        // No output if destination is unreachable
    }
}

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct City {
    name: String,
    lpg_station: bool,
}

#[derive(Debug)]
struct Road {
    destination: usize,
    distance: i32,
}

#[derive(Debug)]
struct Graph {
    cities: Vec<City>,
    roads: Vec<Vec<Road>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            cities: Vec::new(),
            roads: Vec::new(),
        }
    }

    fn add_city(&mut self, name: String, has_lpg: bool) {
        self.cities.push(City { name, lpg_station: has_lpg });
        self.roads.push(Vec::new());
    }

    fn add_road(&mut self, from: usize, to: usize, distance: i32) {
        self.roads[from].push(Road { destination: to, distance });
        self.roads[to].push(Road { destination: from, distance }); // Assuming undirected roads
    }

    fn shortest_path(&self, start: usize, goal: usize, fuel_capacity: i32) -> i32 {
        let mut distances = vec![i32::MAX; self.cities.len()];
        let mut priority_queue = BinaryHeap::new();

        distances[start] = 0;
        priority_queue.push(State { city: start, distance: 0, fuel: fuel_capacity });

        while let Some(State { city, distance, fuel }) = priority_queue.pop() {
            if city == goal {
                return distance;
            }

            if distance > distances[city] {
                continue; // Skip if we found a better path
            }

            for road in &self.roads[city] {
                let new_distance = distance + road.distance;
                let new_fuel = fuel - road.distance;

                if new_fuel >= 0 && new_distance < distances[road.destination] {
                    distances[road.destination] = new_distance;
                    priority_queue.push(State { city: road.destination, distance: new_distance, fuel: new_fuel });
                }
            }
        }

        -1 // Return -1 if no valid path exists
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    city: usize,
    distance: i32,
    fuel: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Example usage
    let mut graph = Graph::new();
    graph.add_city("A".to_string(), true);
    graph.add_city("B".to_string(), false);
    graph.add_city("C".to_string(), true);
    graph.add_road(0, 1, 10);
    graph.add_road(1, 2, 20);
    graph.add_road(2, 0, 15);

    let min_distance = graph.shortest_path(0, 1, 15);
    println!("{}", min_distance); // Should print the minimum distance
}

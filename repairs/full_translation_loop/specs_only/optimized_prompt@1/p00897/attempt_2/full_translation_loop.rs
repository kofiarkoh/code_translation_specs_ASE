use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct City {
    name: String,
    fuel_station: bool,
}

#[derive(Debug)]
struct Road {
    destination: String,
    distance: u32,
}

#[derive(Debug)]
struct Graph {
    cities: HashMap<String, City>,
    roads: HashMap<String, Vec<Road>>,
    fuel_capacity: u32,
}

impl Graph {
    fn new(fuel_capacity: u32) -> Self {
        Self {
            cities: HashMap::new(),
            roads: HashMap::new(),
            fuel_capacity,
        }
    }

    fn add_city(&mut self, name: String, fuel_station: bool) {
        self.cities.insert(name.clone(), City { name, fuel_station });
    }

    fn add_road(&mut self, from: String, to: String, distance: u32) {
        self.roads.entry(from).or_insert_with(Vec::new).push(Road { destination: to, distance });
    }

    fn shortest_path(&self, start: &str, destination: &str) -> i32 {
        if !self.cities.contains_key(start) || !self.cities.contains_key(destination) {
            return -1; // Invalid start or destination
        }

        let mut min_heap = BinaryHeap::new();
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();

        distances.insert(start.to_string(), 0);
        min_heap.push(State { city: start.to_string(), time: 0 });

        while let Some(State { city, time }) = min_heap.pop() {
            if visited.contains(&city) {
                continue;
            }
            visited.insert(city.clone());

            if city == destination {
                return time as i32; // Found the destination
            }

            if let Some(roads) = self.roads.get(&city) {
                for road in roads {
                    let next_city = &road.destination;
                    let travel_time = road.distance;

                    // Check if we can reach the next city with the current fuel capacity
                    if self.cities.get(next_city).is_some() && travel_time <= self.fuel_capacity {
                        let next_time = time + travel_time;
                        let entry = distances.entry(next_city.clone()).or_insert(u32::MAX);
                        if next_time < *entry {
                            *entry = next_time;
                            min_heap.push(State { city: next_city.clone(), time: next_time });
                        }
                    }
                }
            }
        }

        -1 // Destination is unreachable
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    city: String,
    time: u32,
}

// Implementing Ord and PartialOrd for State to use in BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Example usage
fn main() {
    let mut graph = Graph::new(100); // Example fuel capacity
    graph.add_city("A".to_string(), true);
    graph.add_city("B".to_string(), false);
    graph.add_city("C".to_string(), true);
    graph.add_road("A".to_string(), "B".to_string(), 10);
    graph.add_road("B".to_string(), "C".to_string(), 20);
    graph.add_road("C".to_string(), "A".to_string(), 15);

    let min_time = graph.shortest_path("A", "C");
    println!("{}", min_time); // Should print the minimum travel time
}

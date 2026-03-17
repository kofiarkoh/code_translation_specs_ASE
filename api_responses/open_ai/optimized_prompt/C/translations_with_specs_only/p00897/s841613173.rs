use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct City {
    name: String,
    distance: u32,
}

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<String, Vec<City>>,
    lpg_stations: HashSet<String>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            lpg_stations: HashSet::new(),
        }
    }

    fn add_city(&mut self, city_name: String) {
        self.adjacency_list.insert(city_name.clone(), Vec::new());
    }

    fn add_road(&mut self, from: &str, to: &str, distance: u32) {
        if let Some(cities) = self.adjacency_list.get_mut(from) {
            cities.push(City {
                name: to.to_string(),
                distance,
            });
        }
    }

    fn add_lpg_station(&mut self, city_name: String) {
        self.lpg_stations.insert(city_name);
    }

    fn shortest_path(&self, start: &str, goal: &str, fuel_capacity: u32) -> Result<u32, u32> {
        let mut distances: HashMap<String, u32> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: Vec<(String, u32, u32)> = vec![(start.to_string(), 0, fuel_capacity)];

        while let Some((current_city, current_distance, remaining_fuel)) = queue.pop() {
            if current_city == goal {
                return Ok(current_distance);
            }

            if visited.contains(&current_city) {
                continue;
            }
            visited.insert(current_city.clone());

            if let Some(neighbors) = self.adjacency_list.get(&current_city) {
                for neighbor in neighbors {
                    if remaining_fuel >= neighbor.distance {
                        let new_distance = current_distance + neighbor.distance;
                        let new_remaining_fuel = remaining_fuel - neighbor.distance;

                        queue.push((neighbor.name.clone(), new_distance, new_remaining_fuel));
                    }
                }
            }
        }

        Err(u32::MAX) // Return a specific value indicating the journey is not possible
    }
}

fn main() {
    // Example usage
    let mut graph = Graph::new();
    graph.add_city("A".to_string());
    graph.add_city("B".to_string());
    graph.add_city("C".to_string());
    graph.add_road("A", "B", 10);
    graph.add_road("B", "C", 15);
    graph.add_lpg_station("B".to_string());

    let fuel_capacity = 20;
    match graph.shortest_path("A", "C", fuel_capacity) {
        Ok(distance) => println!("Minimum distance: {}", distance),
        Err(_) => println!("Journey not possible"),
    }
}

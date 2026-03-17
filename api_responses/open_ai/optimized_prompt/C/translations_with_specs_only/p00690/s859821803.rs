use std::collections::{HashMap, HashSet};

struct Connection {
    station_a: usize,
    station_b: usize,
    distance: usize,
}

struct RailwayNetwork {
    stations: usize,
    connections: Vec<Connection>,
}

impl RailwayNetwork {
    fn longest_path(&self) -> (usize, Vec<usize>) {
        // Check preconditions
        if self.stations == 0 || self.connections.is_empty() {
            return (0, Vec::new());
        }

        // Build the adjacency list
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        for conn in &self.connections {
            graph.entry(conn.station_a).or_default().push((conn.station_b, conn.distance));
            graph.entry(conn.station_b).or_default().push((conn.station_a, conn.distance));
        }

        let mut max_length = 0;
        let mut longest_path = Vec::new();

        // Helper function for DFS
        fn dfs(
            station: usize,
            graph: &HashMap<usize, Vec<(usize, usize)>>,
            visited: &mut HashSet<usize>,
            current_path: &mut Vec<usize>,
            current_length: usize,
            max_length: &mut usize,
            longest_path: &mut Vec<usize>,
        ) {
            visited.insert(station);
            current_path.push(station);

            // Check if the current path is the longest
            if current_length > *max_length {
                *max_length = current_length;
                *longest_path = current_path.clone();
            }

            // Explore neighbors
            if let Some(neighbors) = graph.get(&station) {
                for &(neighbor, distance) in neighbors {
                    if !visited.contains(&neighbor) {
                        dfs(
                            neighbor,
                            graph,
                            visited,
                            current_path,
                            current_length + distance,
                            max_length,
                            longest_path,
                        );
                    }
                }
            }

            // Backtrack
            visited.remove(&station);
            current_path.pop();
        }

        // Start DFS from each station
        for station in 0..self.stations {
            let mut visited = HashSet::new();
            let mut current_path = Vec::new();
            dfs(station, &graph, &mut visited, &mut current_path, 0, &mut max_length, &mut longest_path);
        }

        (max_length, longest_path)
    }
}

fn main() {
    // Example usage
    let network = RailwayNetwork {
        stations: 5,
        connections: vec![
            Connection { station_a: 0, station_b: 1, distance: 10 },
            Connection { station_a: 1, station_b: 2, distance: 20 },
            Connection { station_a: 0, station_b: 2, distance: 15 },
            Connection { station_a: 2, station_b: 3, distance: 25 },
            Connection { station_a: 3, station_b: 4, distance: 30 },
        ],
    };

    let (length, path) = network.longest_path();
    println!("Longest path length: {}", length);
    println!("Path: {:?}", path);
}

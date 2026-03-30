use std::collections::{HashMap, HashSet};

struct RailwayNetwork {
    stations: usize,
    connections: Vec<(usize, usize, usize)>, // (station1, station2, distance)
}

impl RailwayNetwork {
    fn longest_cyclic_path(&self) -> (usize, Vec<usize>) {
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        
        // Build the undirected graph
        for (s1, s2, distance) in &self.connections {
            graph.entry(*s1).or_default().push((*s2, *distance));
            graph.entry(*s2).or_default().push((*s1, *distance));
        }

        let mut max_length = 0;
        let mut best_path = Vec::new();
        
        // Explore all paths starting from each station
        for &start in 0..self.stations {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            self.dfs(start, start, 0, &mut visited, &mut path, &mut max_length, &mut best_path, &graph);
        }

        (max_length, best_path)
    }

    fn dfs(
        &self,
        current: usize,
        start: usize,
        current_length: usize,
        visited: &mut HashSet<usize>,
        path: &mut Vec<usize>,
        max_length: &mut usize,
        best_path: &mut Vec<usize>,
        graph: &HashMap<usize, Vec<(usize, usize)>>,
    ) {
        visited.insert(current);
        path.push(current);

        // Check if we can form a cycle
        if path.len() > 1 && current == start {
            if current_length > *max_length {
                *max_length = current_length;
                *best_path = path.clone();
            }
        }

        // Explore neighbors
        if let Some(neighbors) = graph.get(&current) {
            for &(neighbor, distance) in neighbors {
                if !visited.contains(&neighbor) || neighbor == start {
                    let new_length = current_length + distance;
                    self.dfs(neighbor, start, new_length, visited, path, max_length, best_path, graph);
                }
            }
        }

        // Backtrack
        visited.remove(&current);
        path.pop();
    }
}

fn main() {
    // Example usage
    let network = RailwayNetwork {
        stations: 4,
        connections: vec![(0, 1, 10), (1, 2, 15), (2, 0, 20), (1, 3, 25)],
    };

    let (length, path) = network.longest_cyclic_path();
    println!("Longest cyclic path length: {}", length);
    println!("Path: {:?}", path);
}

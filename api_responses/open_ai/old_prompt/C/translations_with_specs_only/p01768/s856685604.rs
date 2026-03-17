use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX_NAME_LENGTH: usize = 10;

struct Graph {
    adj_list: HashMap<String, Vec<String>>,
    costs: HashMap<String, i32>,
    visited: HashMap<String, bool>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
            costs: HashMap::new(),
            visited: HashMap::new(),
        }
    }

    fn add_edge(&mut self, s: &str, t: &str) {
        self.adj_list.entry(s.to_string()).or_insert_with(Vec::new).push(t.to_string());
        self.adj_list.entry(t.to_string()).or_insert_with(Vec::new).push(s.to_string());
    }

    fn dfs(&mut self, ingredient: &str) -> i32 {
        if let Some(&visited) = self.visited.get(ingredient) {
            if visited {
                return 0; // Already visited, return 0 cost
            }
        }

        self.visited.insert(ingredient.to_string(), true);
        let mut total_cost = self.costs.get(ingredient).cloned().unwrap_or(0);
        
        if let Some(neighbors) = self.adj_list.get(ingredient) {
            for neighbor in neighbors {
                total_cost += self.dfs(neighbor);
            }
        }
        
        total_cost
    }

    fn compute_min_cost(&mut self) -> i32 {
        let mut total_min_cost = 0;

        for ingredient in self.costs.keys() {
            if !self.visited.get(ingredient).unwrap_or(&false) {
                total_min_cost += self.dfs(ingredient);
            }
        }

        total_min_cost
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut graph = Graph::new();
    let mut n = 0;
    let mut m = 0;

    // Read number of ingredients and connections
    for line in reader.lines().take(2) {
        let line = line.unwrap();
        if n == 0 {
            n = line.parse().unwrap();
        } else {
            m = line.parse().unwrap();
        }
    }

    // Read ingredient costs
    for _ in 0..n {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let name = parts[0];
        let cost: i32 = parts[1].parse().unwrap();
        
        // Store the cost of the ingredient
        graph.costs.insert(name.to_string(), cost);
    }

    // Read connections
    for _ in 0..m {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let s = parts[0];
        let t = parts[1];
        
        graph.add_edge(s, t);
    }

    // Compute and print the total minimum cost
    let total_min_cost = graph.compute_min_cost();
    println!("{}", total_min_cost);
}

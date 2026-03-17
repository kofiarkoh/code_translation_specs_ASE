use std::collections::HashMap;
use std::io::{self, Write};

struct Ingredient {
    name: String,
    cost: u32,
    connections: Vec<String>,
}

struct Graph {
    ingredients: HashMap<String, Ingredient>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            ingredients: HashMap::new(),
        }
    }

    fn add_ingredient(&mut self, name: &str, cost: u32) {
        // Add an ingredient with its cost
        self.ingredients.insert(
            name.to_string(),
            Ingredient {
                name: name.to_string(),
                cost,
                connections: Vec::new(),
            },
        );
    }

    fn add_connection(&mut self, from: &str, to: &str) {
        // Add a connection between two ingredients
        if let Some(ingredient) = self.ingredients.get_mut(from) {
            ingredient.connections.push(to.to_string());
        }
        // If the ingredient is not found, behavior is undefined as per specification
    }

    fn compute_min_cost(&self) -> u32 {
        let mut total_min_cost = 0;

        for ingredient in self.ingredients.values() {
            let min_cost = self.compute_cost_for_ingredient(ingredient);
            total_min_cost += min_cost;
        }

        total_min_cost
    }

    fn compute_cost_for_ingredient(&self, ingredient: &Ingredient) -> u32 {
        let mut min_cost = ingredient.cost;

        for connection in &ingredient.connections {
            if let Some(conn_ingredient) = self.ingredients.get(connection) {
                min_cost = min_cost.min(conn_ingredient.cost);
            }
        }

        min_cost
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_ingredients: usize = input.trim().parse().unwrap();

    let mut graph = Graph::new();

    for _ in 0..num_ingredients {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let name = parts[0];
        let cost: u32 = parts[1].parse().unwrap();
        graph.add_ingredient(name, cost);
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let num_connections: usize = input.trim().parse().unwrap();

    for _ in 0..num_connections {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let from = parts[0];
        let to = parts[1];
        graph.add_connection(from, to);
    }

    let total_min_cost = graph.compute_min_cost();
    println!("{}", total_min_cost);
}

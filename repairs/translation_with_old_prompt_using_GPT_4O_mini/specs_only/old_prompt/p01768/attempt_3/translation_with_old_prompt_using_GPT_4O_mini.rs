use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable
    
    // Read the number of ingredients (vertices) and connections (edges)
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Edge case: if n is 0, print 0 and return
    if n == 0 {
        println!("0");
        return;
    }

    // Initialize adjacency list and cost array
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut ing: Vec<u32> = vec![u32::MAX; n]; // Use MAX to represent uninitialized costs
    let mut visited: HashSet<String> = HashSet::new();

    // Read connections
    for _ in 0..m {
        let mut connection = String::new();
        reader.read_line(&mut connection).unwrap();
        let mut names = connection.split_whitespace();
        let s = names.next().unwrap().to_string();
        let t = names.next().unwrap().to_string();

        // Build the undirected graph
        graph.entry(s.clone()).or_insert_with(Vec::new).push(t.clone());
        graph.entry(t).or_insert_with(Vec::new).push(s);
    }

    // Read ingredient costs
    for i in 0..n {
        let mut cost_line = String::new();
        reader.read_line(&mut cost_line).unwrap();
        let parts: Vec<&str> = cost_line.split_whitespace().collect();
        let name = parts[0].to_string();
        let cost: u32 = parts[1].parse().unwrap();
        
        // Store the cost for the ingredient
        ing[i] = cost;
        visited.insert(name.clone());
    }

    // Function to perform DFS and calculate minimum cost
    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        ing: &Vec<u32>,
        visited: &mut HashSet<String>,
        node: &String,
    ) -> u32 {
        if visited.contains(node) {
            return 0; // Already visited
        }
        visited.insert(node.clone());

        let mut min_cost = ing[node.parse::<usize>().unwrap()]; // Get the cost of the current ingredient
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                min_cost = min_cost.min(dfs(graph, ing, visited, neighbor));
            }
        }
        min_cost
    }

    // Calculate total minimum cost
    let mut total_min_cost = 0;
    let mut visited_for_dfs = HashSet::new(); // Separate visited set for DFS
    for name in visited.iter() {
        if !visited_for_dfs.contains(name) {
            total_min_cost += dfs(&graph, &ing, &mut visited_for_dfs, name);
        }
    }

    // Output the total minimum cost
    println!("{}", total_min_cost);
}

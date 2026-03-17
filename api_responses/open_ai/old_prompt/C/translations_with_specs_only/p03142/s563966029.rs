use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Write};

fn topological_sort(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, &'static str> {
    // Create a graph representation and in-degree count
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree = vec![0; n + 1]; // in-degree for nodes 1 to N

    // Build the graph and in-degree array
    for &(a, b) in edges {
        graph.entry(a).or_insert_with(Vec::new).push(b);
        in_degree[b] += 1;
    }

    // Queue for nodes with no incoming edges
    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut top_order = Vec::new();

    // Process nodes in topological order
    while let Some(node) = queue.pop_front() {
        top_order.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Check if topological sort is possible (i.e., no cycles)
    if top_order.len() != n {
        return Err("Graph has at least one cycle or invalid input");
    }

    Ok(top_order)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let params: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if params.len() != 2 {
        eprintln!("Invalid input");
        std::process::exit(1);
    }

    let n = params[0];
    let m = params[1];

    let mut edges = Vec::new();
    for _ in 0..m {
        let mut edge_input = String::new();
        io::stdin().read_line(&mut edge_input).expect("Failed to read line");
        let edge: Vec<usize> = edge_input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if edge.len() != 2 || edge[0] < 1 || edge[0] > n || edge[1] < 1 || edge[1] > n {
            eprintln!("Invalid edge input");
            std::process::exit(1);
        }

        edges.push((edge[0], edge[1]));
    }

    match topological_sort(n, &edges) {
        Ok(order) => {
            for node in order {
                println!("{}", node);
            }
            std::process::exit(0);
        }
        Err(_) => {
            std::process::exit(1);
        }
    }
}

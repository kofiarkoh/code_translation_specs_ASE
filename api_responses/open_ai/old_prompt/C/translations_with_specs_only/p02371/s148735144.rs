use std::collections::{HashMap, HashSet};

fn tyokkei(n: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32 {
    // Create an adjacency list to represent the graph
    let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    
    // Populate the graph with edges and their weights
    for i in 0..(n - 1) {
        graph.entry(a[i]).or_insert_with(Vec::new).push((b[i], c[i]));
        graph.entry(b[i]).or_insert_with(Vec::new).push((a[i], c[i]));
    }

    // Use DFS to find the maximum path weight from node 0
    let mut visited = HashSet::new();
    let mut max_weight = 0;

    // Perform DFS
    fn dfs(node: i32, current_weight: i32, graph: &HashMap<i32, Vec<(i32, i32)>>, visited: &mut HashSet<i32>, max_weight: &mut i32) {
        visited.insert(node);
        *max_weight = (*max_weight).max(current_weight);

        if let Some(neighbors) = graph.get(&node) {
            for &(neighbor, weight) in neighbors {
                if !visited.contains(&neighbor) {
                    dfs(neighbor, current_weight + weight, graph, visited, max_weight);
                }
            }
        }

        visited.remove(&node); // Allow revisiting for other paths
    }

    // Start DFS from node 0
    dfs(0, 0, &graph, &mut visited, &mut max_weight);

    // Print the maximum path weight
    println!("{}", max_weight);
    
    // Return the maximum path weight
    max_weight
}

fn main() {
    // Example usage (this part is not specified in the original task)
    let n = 5;
    let a = [0, 0, 1, 1];
    let b = [1, 2, 3, 4];
    let c = [10, 20, 30, 40];
    
    tyokkei(n, &a, &b, &c);
}

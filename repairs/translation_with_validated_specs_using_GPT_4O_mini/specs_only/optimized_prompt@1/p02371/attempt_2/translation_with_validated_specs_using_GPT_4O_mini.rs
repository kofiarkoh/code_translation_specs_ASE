use std::collections::{HashMap, HashSet, VecDeque};

fn max_cumulative_cost(n: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> i32 {
    // Preconditions check
    assert!(n > 0);
    assert!(a.len() == n - 1 && b.len() == n - 1 && c.len() == n - 1);
    
    // Graph representation using an adjacency list
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    
    for i in 0..(n - 1) {
        graph.entry(a[i]).or_default().push((b[i], c[i]));
        graph.entry(b[i]).or_default().push((a[i], c[i]));
    }

    // Initialize cumulative costs to a large value
    let mut max_costs = vec![i32::MIN; n];
    max_costs[0] = 0; // Starting vertex cost is 0

    // DFS function to explore paths recursively
    fn dfs(
        vertex: usize,
        graph: &HashMap<usize, Vec<(usize, usize)>>,
        max_costs: &mut Vec<i32>,
        current_cost: i32,
        visited: &mut HashSet<usize>,
    ) {
        visited.insert(vertex);
        
        for &(neighbor, cost) in &graph[&vertex] {
            let new_cost = current_cost + cost as i32;
            if new_cost > max_costs[neighbor] {
                max_costs[neighbor] = new_cost;
                if !visited.contains(&neighbor) {
                    dfs(neighbor, graph, max_costs, new_cost, visited);
                }
            }
        }
        
        visited.remove(&vertex);
    }

    // BFS function to explore paths iteratively
    fn bfs(
        start: usize,
        graph: &HashMap<usize, Vec<(usize, usize)>>,
        max_costs: &mut Vec<i32>,
    ) {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        
        while let Some(vertex) = queue.pop_front() {
            for &(neighbor, cost) in &graph[&vertex] {
                let new_cost = max_costs[vertex] + cost as i32;
                if new_cost > max_costs[neighbor] {
                    max_costs[neighbor] = new_cost;
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Use DFS to explore the graph
    let mut visited = HashSet::new();
    dfs(0, &graph, &mut max_costs, 0, &mut visited);

    // Use BFS to ensure all paths are explored
    bfs(0, &graph, &mut max_costs);

    // The maximum cumulative cost from vertex 0 to any other vertex
    max_costs.iter().cloned().max().unwrap_or(0)
}

fn main() {
    // Example usage
    let n = 5;
    let a = vec![0, 0, 1, 1];
    let b = vec![1, 2, 3, 4];
    let c = vec![10, 20, 30, 40];
    
    let result = max_cumulative_cost(n, a, b, c);
    println!("Maximum cumulative cost: {}", result);
}

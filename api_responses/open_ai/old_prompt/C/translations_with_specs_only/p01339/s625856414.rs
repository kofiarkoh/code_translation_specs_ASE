use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N and M
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize graph structures
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut reverse_graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let s: usize = edge_parts.next().unwrap().parse().unwrap();
        let d: usize = edge_parts.next().unwrap().parse().unwrap();
        graph[s].insert(d);
        reverse_graph[d].insert(s);
    }

    // Step 1: Find SCCs using Kosaraju's algorithm
    let mut visited = vec![false; n];
    let mut finish_stack = Vec::new();

    // First DFS to fill the finish stack
    for i in 0..n {
        if !visited[i] {
            dfs(&graph, i, &mut visited, &mut finish_stack);
        }
    }

    // Second DFS to find SCCs
    visited.fill(false);
    let mut components = Vec::new();
    while let Some(v) = finish_stack.pop() {
        if !visited[v] {
            let mut component = Vec::new();
            reverse_dfs(&reverse_graph, v, &mut visited, &mut component);
            components.push(component);
        }
    }

    // Step 2: Count paths through SCCs
    let scc_count = components.len();
    let mut scc_graph = vec![Vec::new(); scc_count];
    let mut scc_indegree = vec![0; scc_count];
    let mut scc_map = vec![0; n];

    // Map vertices to their SCC index
    for (i, component) in components.iter().enumerate() {
        for &vertex in component {
            scc_map[vertex] = i;
        }
    }

    // Build the SCC graph
    for u in 0..n {
        for &v in &graph[u] {
            let scc_u = scc_map[u];
            let scc_v = scc_map[v];
            if scc_u != scc_v {
                scc_graph[scc_u].push(scc_v);
                scc_indegree[scc_v] += 1;
            }
        }
    }

    // Step 3: Count paths using dynamic programming
    let mut path_count = vec![0; scc_count];
    for i in 0..scc_count {
        if scc_indegree[i] == 0 {
            path_count[i] = 1; // Start from SCCs with no incoming edges
        }
    }

    for i in 0..scc_count {
        for &v in &scc_graph[i] {
            path_count[v] = (path_count[v] + path_count[i]) % MOD;
        }
    }

    // The result is the sum of paths from all SCCs
    let result: i64 = path_count.iter().sum::<i64>() % MOD;
    println!("{}", result);
}

// Depth-first search for the original graph
fn dfs(graph: &Vec<HashSet<usize>>, v: usize, visited: &mut Vec<bool>, finish_stack: &mut Vec<usize>) {
    visited[v] = true;
    for &neighbor in &graph[v] {
        if !visited[neighbor] {
            dfs(graph, neighbor, visited, finish_stack);
        }
    }
    finish_stack.push(v);
}

// Depth-first search for the transposed graph
fn reverse_dfs(graph: &Vec<HashSet<usize>>, v: usize, visited: &mut Vec<bool>, component: &mut Vec<usize>) {
    visited[v] = true;
    component.push(v);
    for &neighbor in &graph[v] {
        if !visited[neighbor] {
            reverse_dfs(graph, neighbor, visited, component);
        }
    }
}

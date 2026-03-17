use std::collections::{HashMap, HashSet};

const MOD: usize = 1_000_000_007;

struct Graph {
    vertices: usize,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new(vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        // Preconditions: vertices and edges must be non-negative integers
        assert!(vertices >= 0);
        for &(u, v) in &edges {
            assert!(u < vertices && v < vertices); // Ensure valid vertex indices
        }
        Graph { vertices, edges }
    }

    fn count_scc(&self) -> usize {
        let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); self.vertices];
        for &(u, v) in &self.edges {
            graph[u].insert(v);
        }

        let mut visited = vec![false; self.vertices];
        let mut finish_stack = Vec::new();

        // First DFS to fill the finish stack
        for i in 0..self.vertices {
            if !visited[i] {
                self.dfs_fill_stack(i, &mut visited, &mut finish_stack, &graph);
            }
        }

        // Transpose the graph
        let mut transposed_graph: Vec<HashSet<usize>> = vec![HashSet::new(); self.vertices];
        for &(u, v) in &self.edges {
            transposed_graph[v].insert(u);
        }

        // Second DFS on the transposed graph
        visited.fill(false);
        let mut scc_count = 0;

        while let Some(v) = finish_stack.pop() {
            if !visited[v] {
                self.dfs_count_scc(v, &mut visited, &transposed_graph);
                scc_count += 1; // Count each SCC found
            }
        }

        scc_count
    }

    fn dfs_fill_stack(&self, v: usize, visited: &mut Vec<bool>, finish_stack: &mut Vec<usize>, graph: &Vec<HashSet<usize>>) {
        visited[v] = true;
        for &neighbor in &graph[v] {
            if !visited[neighbor] {
                self.dfs_fill_stack(neighbor, visited, finish_stack, graph);
            }
        }
        finish_stack.push(v);
    }

    fn dfs_count_scc(&self, v: usize, visited: &mut Vec<bool>, graph: &Vec<HashSet<usize>>) {
        visited[v] = true;
        for &neighbor in &graph[v] {
            if !visited[neighbor] {
                self.dfs_count_scc(neighbor, visited, graph);
            }
        }
    }

    fn distinct_traversal_ways(&self) -> usize {
        let scc_count = self.count_scc();
        // The number of distinct ways to traverse is 2^(scc_count) - 1
        // We use modular exponentiation to compute this
        let mut result = 1;
        let mut base = 2;
        let mut exp = scc_count;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % MOD;
            }
            base = (base * base) % MOD;
            exp /= 2;
        }

        (result + MOD - 1) % MOD // Subtract 1 to exclude the empty traversal
    }
}

fn main() {
    // Example usage:
    let vertices = 5;
    let edges = vec![(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)];
    let graph = Graph::new(vertices, edges);
    let result = graph.distinct_traversal_ways();
    println!("{}", result); // Output the number of distinct traversal ways
}

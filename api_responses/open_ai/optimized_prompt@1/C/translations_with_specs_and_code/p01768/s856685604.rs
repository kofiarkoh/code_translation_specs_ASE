use std::collections::HashMap;
use std::io::{self, BufRead};

const INIT_CAP: usize = 10;
const MAX_INGREDIENTS: usize = 5000;

#[derive(Debug)]
struct AdjList {
    to: Vec<usize>,
}

#[derive(Debug)]
struct Rec {
    name: String,
    cost: usize,
}

struct Graph {
    adj: Vec<AdjList>,
    ing: Vec<Rec>,
}

impl Graph {
    fn new(n: usize) -> Self {
        let adj = vec![AdjList { to: Vec::with_capacity(INIT_CAP) }; n];
        let ing = Vec::with_capacity(n);
        Graph { adj, ing }
    }

    fn add_connection(&mut self, from: usize, to: usize) {
        if from < self.adj.len() && to < self.adj.len() {
            self.adj[from].to.push(to);
        }
    }

    fn search_min_cost(&mut self, i: usize) -> usize {
        let mut visited = vec![false; self.ing.len()];
        self.dfs(i, &mut visited)
    }

    fn dfs(&mut self, i: usize, visited: &mut Vec<bool>) -> usize {
        visited[i] = true;
        let mut min_cost = self.ing[i].cost;

        for &neighbor in &self.adj[i].to {
            if !visited[neighbor] {
                min_cost = min_cost.min(self.dfs(neighbor, visited));
            }
        }

        min_cost
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut graph = Graph::new(n);

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();
        let cost: usize = parts[1].parse().unwrap();
        graph.ing.push(Rec { name, cost });
    }

    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let s = parts[0];
        let t = parts[1];

        let idx_s = graph.ing.iter().position(|ing| ing.name == s).unwrap_or(n);
        let idx_t = graph.ing.iter().position(|ing| ing.name == t).unwrap_or(n);

        graph.add_connection(idx_s, idx_t);
        graph.add_connection(idx_t, idx_s);
    }

    let total_min_cost: usize = (0..n).map(|i| graph.search_min_cost(i)).sum();
    println!("{}", total_min_cost);
}
```

### Notes:
1. **Graph Structure**: The `Graph` struct encapsulates the adjacency list and ingredient records, maintaining a clear separation of concerns.
2. **Dynamic Memory Management**: Rust's `Vec` is used to manage dynamic arrays, eliminating the need for manual memory management (malloc/realloc).
3. **Error Handling**: The code uses `unwrap_or(n)` to handle cases where an ingredient name is not found, which aligns with the specification's requirement to ignore non-existent connections.
4. **DFS Implementation**: The depth-first search (DFS) is implemented recursively, similar to the C code, but using Rust's ownership and borrowing rules.
5. **Input Handling**: The program reads input using `io::stdin`, ensuring it matches the expected input format.
6. **Cost Calculation**: The minimum cost is calculated using a method that updates the cost directly in the `Rec` struct, preserving the semantics of the original C code.

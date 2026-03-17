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
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        let adj = vec![AdjList { to: Vec::with_capacity(INIT_CAP) }; n];
        let ing = Vec::with_capacity(n);
        Graph { adj, ing, n }
    }

    fn add_connection(&mut self, from: usize, to: usize) {
        self.adj[from].to.push(to);
    }

    fn search_min_cost(&mut self, i: usize) -> usize {
        let mut visited = vec![false; self.n];
        let mut min_cost = usize::MAX;

        self.dfs(i, &mut visited, &mut min_cost);
        self.ing[i].cost = min_cost;
        min_cost
    }

    fn dfs(&self, i: usize, visited: &mut Vec<bool>, min_cost: &mut usize) {
        visited[i] = true;
        *min_cost = (*min_cost).min(self.ing[i].cost);

        for &to in &self.adj[i].to {
            if !visited[to] {
                self.dfs(to, visited, min_cost);
            }
        }
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
        let idx_s = graph.ing.iter().position(|ing| ing.name == parts[0]).unwrap();
        let idx_t = graph.ing.iter().position(|ing| ing.name == parts[1]).unwrap();

        graph.add_connection(idx_s, idx_t);
        graph.add_connection(idx_t, idx_s);
    }

    let total_min_cost: usize = (0..n).map(|i| graph.search_min_cost(i)).sum();
    println!("{}", total_min_cost);
}

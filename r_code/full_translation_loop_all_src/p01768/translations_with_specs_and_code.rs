use std::io::{self, BufRead};

const INIT_CAP: usize = 10;
const MAX_INGREDIENTS: usize = 5000;

#[derive(Debug, Clone)]
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
        if from < self.n && to < self.n {
            self.adj[from].to.push(to);
            self.adj[to].to.push(from); // Add bidirectional connection
        }
    }

    fn search_idx(&self, name: &str) -> Option<usize> {
        self.ing.iter().position(|rec| rec.name == name)
    }

    fn dfs(&self, i: usize, visited: &mut Vec<bool>) -> usize {
        visited[i] = true;
        let mut min_cost = self.ing[i].cost;

        for &neighbor in &self.adj[i].to {
            if !visited[neighbor] {
                min_cost = min_cost.min(self.dfs(neighbor, visited));
            }
        }

        min_cost
    }

    fn search_min_cost(&self, i: usize) -> usize {
        let mut visited = vec![false; self.n];
        self.dfs(i, &mut visited)
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

        if let (Some(idx_s), Some(idx_t)) = (graph.search_idx(s), graph.search_idx(t)) {
            graph.add_connection(idx_s, idx_t);
        }
    }

    let total_min_cost: usize = (0..n).map(|i| graph.search_min_cost(i)).sum();
    println!("{}", total_min_cost);
}

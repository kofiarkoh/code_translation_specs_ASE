use std::io::{self, BufRead};

const INIT_CAP: usize = 10;

#[derive(Debug, Clone)]
struct AdjList {
    to: Vec<usize>,
    cap: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct Rec {
    name: String,
    cost: i32,
}

struct State {
    n: usize,
    adj: Vec<AdjList>,
    ing: Vec<Rec>,
    min: i32,
    visited: Vec<bool>,
}

impl State {
    fn new(n: usize) -> Self {
        let adj = vec![AdjList { to: Vec::with_capacity(INIT_CAP), cap: INIT_CAP, size: 0 }; n];
        let ing = vec![Rec { name: String::new(), cost: 0 }; n];
        let visited = vec![false; n];
        Self { n, adj, ing, min: 0, visited }
    }

    fn init(&mut self) {
        for i in 0..self.n {
            self.adj[i] = AdjList {
                to: Vec::with_capacity(INIT_CAP),
                cap: INIT_CAP,
                size: 0,
            };
        }
    }

    fn add(&mut self, from: usize, to: usize) {
        if self.adj[from].size == self.adj[from].cap {
            self.adj[from].cap *= 2;
            self.adj[from].to.reserve(self.adj[from].cap - self.adj[from].to.len());
        }
        self.adj[from].to.push(to);
        self.adj[from].size += 1;
    }

    fn search_idx(&self, str: &str) -> usize {
        for i in 0..self.n {
            if self.ing[i].name == str {
                return i;
            }
        }
        self.n // Return n if not found, simulating the behavior of returning an out-of-bounds index
    }

    fn dfs(&mut self, i: usize) {
        self.visited[i] = true;
        self.min = self.min.min(self.ing[i].cost);

        for &to in &self.adj[i].to {
            if !self.visited[to] {
                self.dfs(to);
            }
        }
    }

    fn search_min_cost(&mut self, i: usize) -> i32 {
        self.min = 100000;
        self.visited.fill(false);

        self.dfs(i);

        self.ing[i].cost = self.min;
        self.min
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut state = State::new(n);

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        state.ing[i] = Rec {
            name: parts[0].to_string(),
            cost: parts[1].parse().unwrap(),
        };
    }

    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    state.init();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let idx_s = state.search_idx(parts[0]);
        let idx_t = state.search_idx(parts[1]);

        state.add(idx_s, idx_t);
        state.add(idx_t, idx_s);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += state.search_min_cost(i);
    }

    println!("{}", ans);
}

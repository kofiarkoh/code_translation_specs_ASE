use std::collections::VecDeque;
use std::io::{self, Write};

const MAX: usize = 100005;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: usize,
}

struct Tree {
    edges: Vec<Vec<Edge>>,
    depth: Vec<usize>,
    cost: Vec<usize>,
    depth_max: usize,
}

impl Tree {
    fn new(n: usize) -> Self {
        Self {
            edges: vec![Vec::new(); n + 1],
            depth: vec![0; n + 1],
            cost: vec![0; n + 1],
            depth_max: 0,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.edges[from].push(Edge { to, cost });
        self.edges[to].push(Edge { to: from, cost });
    }

    fn dfs(&mut self) {
        let mut visited = vec![false; self.edges.len()];
        let mut queue = VecDeque::new();
        queue.push_back((1, 0)); // Start from node 1 with depth 0

        while let Some((node, d)) = queue.pop_back() {
            if visited[node] {
                continue;
            }
            visited[node] = true;

            self.depth[node] = d;
            if d > self.depth_max {
                self.depth_max = d;
            }

            for edge in &self.edges[node] {
                if !visited[edge.to] {
                    self.cost[edge.to] = edge.cost;
                    queue.push_back((edge.to, d + 1));
                }
            }
        }
    }
}

fn read_double() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse double: {}", input);
        std::process::exit(1);
    })
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse usize: {}", input);
        std::process::exit(1);
    })
}

fn main() {
    let probability: f64 = read_double();
    let n: usize = read_usize();

    let mut tree = Tree::new(n);
    let mut edges_input = Vec::new();

    for _ in 1..n {
        let x = read_usize();
        let y = read_usize();
        let cost = read_usize();
        edges_input.push((x, y, cost));
        tree.add_edge(x, y, cost);
    }

    tree.dfs();

    let mut p = vec![1.0; tree.depth_max + 1];
    for i in 1..=tree.depth_max {
        p[i] = p[i - 1] * probability;
    }

    let mut a = 0.0;
    let mut b = 1.0;

    for i in 1..=n {
        a += tree.cost[i] as f64 * p[tree.depth[i]];
        b += p[tree.depth[i]];
    }

    println!("{:.10}", a * b);
}

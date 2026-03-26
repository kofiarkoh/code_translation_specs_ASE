use std::collections::VecDeque;
use std::io::{self, Write};

const MAX: usize = 100005;

#[derive(Debug)]
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

    fn dfs(&mut self) {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.edges.len()];

        queue.push_back((1, 0)); // Start from node 1 with depth 0

        while let Some((node, d)) = queue.pop_front() {
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
    input.trim().parse().unwrap()
}

fn read_int() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let p = read_double();
    let n: usize = read_int();

    let mut tree = Tree::new(n);
    let mut edges_input = Vec::new();

    for _ in 1..n {
        let x: usize = read_int();
        let y: usize = read_int();
        let c: usize = read_int();
        edges_input.push((x, y, c));
        tree.edges[x].push(Edge { to: y, cost: c });
        tree.edges[y].push(Edge { to: x, cost: c });
    }

    tree.dfs();

    let mut p_values = vec![1.0; tree.depth_max + 1];
    for i in 1..=tree.depth_max {
        p_values[i] = p_values[i - 1] * p;
    }

    let mut a = 0.0;
    let mut b = 1.0;

    for i in 1..=n {
        a += tree.cost[i] as f64 * p_values[tree.depth[i]];
        b += p_values[tree.depth[i]];
    }

    println!("{:.10}", a * b);
}

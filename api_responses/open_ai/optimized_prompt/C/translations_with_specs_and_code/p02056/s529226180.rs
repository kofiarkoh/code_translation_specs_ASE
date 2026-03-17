use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    v: i32,
    d: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d) // Reverse order for max-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

impl Eq for Node {}

#[derive(Debug)]
struct Graph {
    edges: Vec<(i32, i32)>, // (vertex, cost)
    start: Vec<i32>,
    pointer: usize,
    vertex_num: i32,
    edge_max_size: usize,
}

impl Graph {
    fn new(vertex_num: i32) -> Self {
        let start = vec![-1; vertex_num as usize];
        Self {
            edges: Vec::new(),
            start,
            pointer: 0,
            vertex_num,
            edge_max_size: 1,
        }
    }

    fn add_edge(&mut self, from: i32, to: i32, cost: i32) {
        if self.pointer == self.edge_max_size {
            self.edge_max_size *= 2;
        }
        self.edges.push((to, cost));
        self.start[from as usize] = self.pointer as i32;
        self.pointer += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: i32 = params.next().unwrap().parse().unwrap();
    let m: i32 = params.next().unwrap().parse().unwrap();
    let k: i32 = params.next().unwrap().parse().unwrap();

    let mut p: Vec<i32> = Vec::with_capacity(3 * n as usize);
    for _ in 0..3 * n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        p.push(value);
    }

    let sup = 1_000_000_001;
    let mut graph = Graph::new(2 * n);
    for i in 0..n {
        graph.add_edge(i, i + n, sup - p[i as usize]);
    }

    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_params = edge_line.split_whitespace();
        let a: i32 = edge_params.next().unwrap().parse().unwrap() - 1;
        let b: i32 = edge_params.next().unwrap().parse().unwrap() - 1;
        let d: i32 = edge_params.next().unwrap().parse().unwrap();

        graph.add_edge(a, b, d);
        graph.add_edge(b, a, d);
        graph.add_edge(a + n, b + n, d);
        graph.add_edge(b + n, a + n, d);
    }

    let mut dp = vec![1_000_000_002 * 2 * n as i64; (2 * n) as usize];
    dp[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Node { v: 0, d: 0 });

    while let Some(Node { v, d }) = heap.pop() {
        if d > dp[v as usize] {
            continue;
        }
        let start_index = graph.start[v as usize];
        for i in start_index..graph.pointer as i32 {
            let (u, cost) = graph.edges[i as usize];
            let new_distance = d + cost as i64;
            if new_distance < dp[u as usize] {
                dp[u as usize] = new_distance;
                heap.push(Node { v: u, d: new_distance });
            }
        }
    }

    let mut max = vec![2 - (1_000_000_001 * 2 * n) as i64; (k + 1) as usize];
    for i in 0..n {
        let v = (sup - dp[(i + n) as usize]) - dp[i as usize] + p[(i) as usize];
        max[p[i as usize] as usize] = max[p[i as usize] as usize].max(v);
    }

    for i in 1..=k {
        println!("{}", max[i as usize]);
    }
}

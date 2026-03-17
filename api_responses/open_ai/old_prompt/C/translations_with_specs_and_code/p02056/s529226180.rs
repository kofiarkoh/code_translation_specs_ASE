use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

type I32 = i32;
type I64 = i64;

#[derive(Debug)]
struct GraphEdge {
    vertex: I32,
    next: I32,
    cost: I32,
}

#[derive(Debug)]
struct DirectedGraph {
    edges: Vec<GraphEdge>,
    start: Vec<I32>,
    pointer: I32,
    vertex_num: I32,
    edge_max_size: I32,
}

impl DirectedGraph {
    fn new(vertex_num: I32) -> Self {
        let start = vec![-1; vertex_num as usize];
        Self {
            edges: Vec::with_capacity(1),
            start,
            pointer: 0,
            vertex_num,
            edge_max_size: 1,
        }
    }

    fn add_edge(&mut self, from: I32, to: I32, cost: I32) {
        if self.pointer == self.edge_max_size {
            self.edge_max_size *= 2;
            self.edges.resize(self.edge_max_size as usize, GraphEdge { vertex: 0, next: 0, cost: 0 });
        }
        self.edges[self.pointer as usize] = GraphEdge { vertex: to, next: self.start[from as usize], cost };
        self.start[from as usize] = self.pointer;
        self.pointer += 1;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    v: I32,
    d: I64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d) // Max-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace().map(|x| x.parse::<I32>().unwrap());
    let n = params.next().unwrap();
    let m = params.next().unwrap();
    let k = params.next().unwrap();

    let mut p = vec![0; (3 * n) as usize];
    for i in 0..(3 * n) {
        p[i as usize] = lines.next().unwrap().unwrap().parse::<I32>().unwrap();
    }

    let sup = 1_000_000_001;
    let mut graph = DirectedGraph::new(2 * n);
    for i in 0..n {
        graph.add_edge(i, i + n, sup - p[i as usize]);
    }

    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_params = edge_line.split_whitespace().map(|x| x.parse::<I32>().unwrap());
        let a = edge_params.next().unwrap() - 1;
        let b = edge_params.next().unwrap() - 1;
        let d = edge_params.next().unwrap();

        graph.add_edge(a, b, d);
        graph.add_edge(b, a, d);
        graph.add_edge(a + n, b + n, d);
        graph.add_edge(b + n, a + n, d);
    }

    let mut dp = vec![1_000_000_001 * 2 * n as I64; (2 * n) as usize];
    dp[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Node { v: 0, d: 0 });

    while let Some(Node { v, d }) = heap.pop() {
        if d > dp[v as usize] {
            continue;
        }
        let mut p = graph.start[v as usize];
        while p != -1 {
            let edge = &graph.edges[p as usize];
            let u = edge.vertex;
            let new_d = d + edge.cost as I64;
            if new_d >= dp[u as usize] as I64 {
                p = edge.next;
                continue;
            }
            dp[u as usize] = new_d as I64;
            heap.push(Node { v: u, d: new_d });
            p = edge.next;
        }
    }

    let mut max = vec![2 - (1_000_000_001 * 2 * n) as I64; (k + 1) as usize];
    for i in 0..n {
        let v = (sup - dp[(i + n) as usize]) - dp[i as usize] + p[i as usize];
        max[p[i as usize] as usize] = max[p[i as usize] as usize].max(v);
    }

    for i in 1..=k {
        println!("{}", max[i as usize]);
    }
}

fn main() {
    run();
}

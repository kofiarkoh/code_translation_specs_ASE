use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Edge {
    to: usize,
    next: Option<Box<Edge>>,
}

struct Graph {
    edges: Vec<Option<Box<Edge>>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![None; n + 1], // 1-indexed
        }
    }

    fn push(&mut self, i: usize, j: usize) {
        let new_edge = Box::new(Edge {
            to: j,
            next: self.edges[i].take(),
        });
        self.edges[i] = Some(new_edge);
    }
}

fn bfs(graph: &Graph, n: usize) -> Vec<i32> {
    let mut d = vec![-1; n + 1]; // -1 for unvisited
    let mut queue = VecDeque::new();

    d[1] = 0; // Starting node
    queue.push_back(1);

    while let Some(i) = queue.pop_front() {
        let mut edge = graph.edges[i].as_ref();
        while let Some(e) = edge {
            let j = e.to;
            if d[j] == -1 {
                d[j] = i as i32; // Store predecessor
                queue.push_back(j);
            }
            edge = e.next.as_ref();
        }
    }
    d
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n);

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        graph.push(a, b);
        graph.push(b, a);
    }

    let d = bfs(&graph, n);

    println!("Yes");
    for i in 2..=n {
        println!("{}", d[i]);
    }
}

use std::collections::VecDeque;
use std::io::{self, Write};

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

    fn push(&mut self, from: usize, to: usize) {
        let new_edge = Box::new(Edge {
            to,
            next: self.edges[from].take(),
        });
        self.edges[from] = Some(new_edge);
    }
}

fn bfs(graph: &Graph, n: usize) -> Vec<i32> {
    let mut parent = vec![-1; n + 1]; // -1 indicates unvisited
    let mut queue = VecDeque::new();
    
    parent[1] = 0; // Starting node has no parent
    queue.push_back(1);

    while let Some(current) = queue.pop_front() {
        let mut edge = graph.edges[current].as_ref();
        while let Some(e) = edge {
            let next_node = e.to;
            if parent[next_node] == -1 { // If unvisited
                parent[next_node] = current as i32; // Set parent
                queue.push_back(next_node);
            }
            edge = e.next.as_ref();
        }
    }
    parent
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n);

    for _ in 0..m {
        let mut edge_input = String::new();
        stdin.read_line(&mut edge_input).unwrap();
        let mut edge_iter = edge_input.split_whitespace();
        let a: usize = edge_iter.next().unwrap().parse().unwrap();
        let b: usize = edge_iter.next().unwrap().parse().unwrap();
        graph.push(a, b);
        graph.push(b, a);
    }

    let parent = bfs(&graph, n);

    println!("Yes");
    for i in 2..=n {
        println!("{}", parent[i]);
    }
}

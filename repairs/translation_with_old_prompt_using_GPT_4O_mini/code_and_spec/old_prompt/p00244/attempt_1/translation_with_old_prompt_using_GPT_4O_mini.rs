use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

const INF: i32 = i32::MAX;
const MAX: usize = 5000;

#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cost: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Node {
    cost: i32,
    index: usize,
    free_moves: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse ordering for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); size],
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, cost: i32) {
        self.edges[a].push(Edge { to: b, cost });
        self.edges[b].push(Edge { to: a, cost });
    }
}

fn search(graph: &Graph, start: usize, goal: usize) -> i32 {
    let mut node_costs = vec![vec![-1; 3]; graph.edges.len()];
    let mut heap = BinaryHeap::new();

    node_costs[start][2] = 0;
    heap.push(Node { cost: 0, index: start, free_moves: 2 });

    while let Some(Node { cost, index, free_moves }) = heap.pop() {
        if index == goal {
            if free_moves != 1 {
                break;
            }
        }

        for edge in &graph.edges[index] {
            let new_cost = cost + edge.cost;

            if free_moves != 1 {
                if node_costs[edge.to][free_moves] < 0 || new_cost < node_costs[edge.to][free_moves] {
                    node_costs[edge.to][free_moves] = new_cost;
                    heap.push(Node { cost: new_cost, index: edge.to, free_moves });
                }
            }

            if free_moves > 0 {
                let new_free_moves = free_moves - 1;
                if node_costs[edge.to][new_free_moves] < 0 || cost < node_costs[edge.to][new_free_moves] {
                    node_costs[edge.to][new_free_moves] = cost;
                    heap.push(Node { cost, index: edge.to, free_moves: new_free_moves });
                }
            }
        }
    }

    if node_costs[goal][0] >= 0 && node_costs[goal][2] >= 0 {
        return node_costs[goal][0].min(node_costs[goal][2]);
    }
    if node_costs[goal][0] >= 0 {
        return node_costs[goal][0];
    }
    node_costs[goal][2]
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let size: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        if size == 0 {
            break;
        }

        let mut graph = Graph::new(size);

        for _ in 0..m {
            let edge_line = lines.next().unwrap().unwrap();
            let mut edge_parts = edge_line.split_whitespace();
            let a: usize = edge_parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let b: usize = edge_parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let cost: i32 = edge_parts.next().unwrap().parse().unwrap();
            graph.add_edge(a, b, cost);
        }

        println!("{}", search(&graph, 0, size - 1));
    }
}

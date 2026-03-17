use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

const INF: i32 = i32::MAX;
const MAX_NODES: usize = 102;

#[derive(Debug)]
struct Edge {
    to: usize,
    cost: i32,
}

#[derive(Debug)]
struct Node {
    index: usize,
    cost: i32,
    free_travels: i32,
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

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); size],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: i32) {
        self.edges[from].push(Edge { to, cost });
        self.edges[to].push(Edge { to: from, cost });
    }

    fn search(&self, start: usize, goal: usize) -> i32 {
        let mut costs = vec![vec![-1; 3]; MAX_NODES];
        let mut heap = BinaryHeap::new();

        costs[start][2] = 0;
        heap.push(Node {
            index: start,
            cost: 0,
            free_travels: 2,
        });

        while let Some(Node { index, cost, free_travels }) = heap.pop() {
            if index == goal {
                if free_travels != 1 {
                    break;
                }
            }

            for edge in &self.edges[index] {
                let next_cost = cost + edge.cost;

                if free_travels != 1 {
                    if costs[edge.to][free_travels] < 0 || next_cost < costs[edge.to][free_travels] {
                        costs[edge.to][free_travels] = next_cost;
                        heap.push(Node {
                            index: edge.to,
                            cost: next_cost,
                            free_travels,
                        });
                    }
                }

                if free_travels > 0 {
                    let next_free_travels = free_travels - 1;
                    if costs[edge.to][next_free_travels as usize] < 0 || cost < costs[edge.to][next_free_travels as usize] {
                        costs[edge.to][next_free_travels as usize] = cost;
                        heap.push(Node {
                            index: edge.to,
                            cost,
                            free_travels: next_free_travels,
                        });
                    }
                }
            }
        }

        if costs[goal][0] >= 0 && costs[goal][2] >= 0 {
            costs[goal][0].min(costs[goal][2])
        } else if costs[goal][0] >= 0 {
            costs[goal][0]
        } else {
            costs[goal][2]
        }
    }
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

        println!("{}", graph.search(0, size - 1));
    }
}

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

const INF: i32 = i32::MAX;
const MAX_NODES: usize = 102;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i32,
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct State {
    cost: i32,
    node: usize,
    free_transitions: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node && self.free_transitions == other.free_transitions
    }
}

impl Eq for State {}

fn search(start: usize, goal: usize, nodes: &[Node], _max_free_transitions: usize) -> i32 {
    let mut dist = vec![vec![INF; 3]; MAX_NODES];
    let mut heap = BinaryHeap::new();

    dist[start][2] = 0;
    heap.push(State { cost: 0, node: start, free_transitions: 2 });

    while let Some(State { cost, node, free_transitions }) = heap.pop() {
        if node == goal {
            return cost; // Return the cost when reaching the goal
        }

        for edge in &nodes[node].edges {
            let next_node = edge.to;
            let next_cost = cost + edge.cost;

            if free_transitions != 0 {
                if dist[next_node][free_transitions] > next_cost {
                    dist[next_node][free_transitions] = next_cost;
                    heap.push(State { cost: next_cost, node: next_node, free_transitions });
                }
            }

            if free_transitions > 0 {
                let next_free_transitions = free_transitions - 1;
                if dist[next_node][next_free_transitions] > cost {
                    dist[next_node][next_free_transitions] = cost;
                    heap.push(State { cost, node: next_node, free_transitions: next_free_transitions });
                }
            }
        }
    }

    // Return the minimum cost to reach the goal with any number of free transitions
    let min_cost = dist[goal].iter().cloned().min().unwrap_or(INF);
    if min_cost == INF { -1 } else { min_cost }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let mut parts = line.split_whitespace();
        let size: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        if size == 0 {
            break;
        }

        let mut nodes = vec![Node { edges: Vec::new() }; size];

        for _ in 0..m {
            if let Some(Ok(line)) = lines.next() {
                let mut parts = line.split_whitespace();
                let a: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                let b: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                let cost: i32 = parts.next().unwrap().parse().unwrap();

                nodes[a].edges.push(Edge { to: b, cost });
                nodes[b].edges.push(Edge { to: a, cost });
            }
        }

        let result = search(0, size - 1, &nodes, 2);
        println!("{}", if result == INF { 0 } else { result });
    }
}

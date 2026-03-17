use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    target: usize,
    weight: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    id: usize,
    x: i32,
    y: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: usize, goal: usize, graph: &[Vec<Edge>]) -> i64 {
    let mut distances = vec![i64::MAX; graph.len()];
    let mut heap = BinaryHeap::new();
    
    distances[start] = 0;
    heap.push((0, start)); // (distance, node)

    while let Some((current_distance, current_node)) = heap.pop() {
        if current_node == goal {
            return current_distance;
        }

        if current_distance > distances[current_node] {
            continue;
        }

        for edge in &graph[current_node] {
            let next_distance = current_distance + edge.weight;
            if next_distance < distances[edge.target] {
                distances[edge.target] = next_distance;
                heap.push((next_distance, edge.target));
            }
        }
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let m = dimensions.next().unwrap();
    let n = dimensions.next().unwrap();
    let k = dimensions.next().unwrap();

    let mut nodes = Vec::new();
    let mut graph = vec![Vec::new(); (2 * k + 2) as usize];
    let mut start = None;
    let mut goal = None;

    for i in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let id = (i * 2) as usize;

        nodes.push(Node { id, x, y });
        if x == 1 && y == 1 {
            start = Some(id);
        } else if x == m && y == n {
            goal = Some(id);
        }
    }

    let start_id = start.unwrap_or_else(|| {
        let id = nodes.len() * 2;
        nodes.push(Node { id, x: 1, y: 1 });
        id
    });

    let goal_id = goal.unwrap_or_else(|| {
        let id = nodes.len() * 2;
        nodes.push(Node { id, x: m, y: n });
        id
    });

    nodes.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

    for i in 0..nodes.len() {
        let a = nodes[i].id;
        let b = a + 1;
        graph[a].push(Edge { target: b, weight: 1 });
        graph[b].push(Edge { target: a, weight: 1 });

        if i > 0 && nodes[i - 1].x == nodes[i].x {
            let d = (nodes[i - 1].y - nodes[i].y).abs();
            graph[nodes[i - 1].id].push(Edge { target: nodes[i].id, weight: d as i64 });
            graph[nodes[i].id].push(Edge { target: nodes[i - 1].id, weight: d as i64 });
        }
    }

    nodes.sort_by(|a, b| a.y.cmp(&b.y));

    for i in 0..nodes.len() {
        if i > 0 && nodes[i - 1].y == nodes[i].y {
            let a = nodes[i - 1].id + 1;
            let b = nodes[i].id + 1;
            let d = (nodes[i - 1].x - nodes[i].x).abs();
            graph[a].push(Edge { target: b, weight: d as i64 });
            graph[b].push(Edge { target: a, weight: d as i64 });
        }
    }

    let result = dijkstra(start_id, goal_id, &graph);
    println!("{}", result);
}

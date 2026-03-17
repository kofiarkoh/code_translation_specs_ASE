use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    target: usize,
    weight: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    index: usize,
    distance: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse order for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const INF: usize = usize::MAX;

fn dijkstra(start: usize, goal: usize, graph: &[Vec<Edge>]) -> isize {
    let mut distances = vec![INF; graph.len()];
    let mut heap = BinaryHeap::new();
    
    distances[start] = 0;
    heap.push(Node { index: start, distance: 0 });

    while let Some(Node { index, distance }) = heap.pop() {
        if distance > distances[index] {
            continue;
        }
        if index == goal {
            return distance as isize;
        }
        for edge in &graph[index] {
            let next = edge.target;
            let new_distance = distance + edge.weight;
            if new_distance < distances[next] {
                distances[next] = new_distance;
                heap.push(Node { index: next, distance: new_distance });
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
    let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let m = dimensions.next().unwrap();
    let n = dimensions.next().unwrap();
    let k = dimensions.next().unwrap();

    let mut points = Vec::new();
    let mut graph = vec![Vec::new(); 2 * k + 2]; // 2 for each point (in and out)

    let mut start = None;
    let mut goal = None;

    for i in 0..k {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let x = coords[0];
        let y = coords[1];
        let id = 2 * i;

        points.push(Point { x, y, id });
        if x == 1 && y == 1 {
            start = Some(id);
        }
        if x == m && y == n {
            goal = Some(id);
        }
    }

    let start = start.unwrap_or_else(|| {
        points.push(Point { x: 1, y: 1, id: 2 * k });
        2 * k
    });

    let goal = goal.unwrap_or_else(|| {
        points.push(Point { x: m, y: n, id: 2 * k + 1 });
        2 * k + 1
    });

    points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    for i in 0..points.len() - 1 {
        let a = points[i].id;
        let b = points[i + 1].id;
        let weight = (points[i + 1].y as isize - points[i].y as isize).abs() as usize;

        graph[a].push(Edge { target: b, weight });
        graph[b].push(Edge { target: a, weight });
    }

    points.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    for i in 0..points.len() - 1 {
        let a = points[i].id + 1;
        let b = points[i + 1].id + 1;
        let weight = (points[i + 1].x as isize - points[i].x as isize).abs() as usize;

        graph[a].push(Edge { target: b, weight });
        graph[b].push(Edge { target: a, weight });
    }

    let result = dijkstra(start, goal, &graph);
    println!("{}", result);
}

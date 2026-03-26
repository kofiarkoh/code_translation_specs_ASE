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
    weight: u64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight) // Reverse order for min-heap
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: usize, goal: usize, graph: &Vec<Vec<Edge>>, used: &mut Vec<bool>) -> i64 {
    let mut heap = BinaryHeap::new();
    let mut distances = vec![u64::MAX; graph.len()];
    distances[start] = 0;
    heap.push(Edge { target: start, weight: 0 });

    while let Some(Edge { target, weight }) = heap.pop() {
        if target == goal {
            return weight as i64;
        }
        if used[target] {
            continue;
        }
        used[target] = true;

        for edge in &graph[target] {
            if !used[edge.target] {
                let new_distance = weight + edge.weight;
                if new_distance < distances[edge.target] {
                    distances[edge.target] = new_distance;
                    heap.push(Edge { target: edge.target, weight: new_distance });
                }
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
    let mut params = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let m = params.next().unwrap();
    let n = params.next().unwrap();
    let k = params.next().unwrap();

    let mut points = Vec::new();
    let mut graph = vec![Vec::new(); 2 * k + 2];
    let mut used = vec![false; 2 * k + 2];
    let mut start = None;
    let mut goal = None;

    for i in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        points.push(Point { x, y, id: 2 * i });

        if x == 1 && y == 1 {
            start = Some(2 * i);
        }
        if x == m && y == n {
            goal = Some(2 * i + 1); // Adjusted to match the correct ID for the goal
        }
    }

    if start.is_none() {
        points.push(Point { x: 1, y: 1, id: 2 * k });
        start = Some(2 * k);
    }
    if goal.is_none() {
        points.push(Point { x: m, y: n, id: 2 * k + 1 });
        goal = Some(2 * k + 1);
    }

    points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    for i in 0..points.len() - 1 {
        let a = points[i].id;
        let b = points[i + 1].id;
        let distance = (points[i].y as isize - points[i + 1].y as isize).abs() as u64;
        graph[a].push(Edge { target: b, weight: distance });
        graph[b].push(Edge { target: a, weight: distance });
    }

    points.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    for i in 0..points.len() - 1 {
        let a = points[i].id + 1;
        let b = points[i + 1].id + 1;
        let distance = (points[i].x as isize - points[i + 1].x as isize).abs() as u64;
        graph[a].push(Edge { target: b, weight: distance });
        graph[b].push(Edge { target: a, weight: distance });
    }

    let result = dijkstra(start.unwrap(), goal.unwrap(), &graph, &mut used);
    println!("{}", result);
}

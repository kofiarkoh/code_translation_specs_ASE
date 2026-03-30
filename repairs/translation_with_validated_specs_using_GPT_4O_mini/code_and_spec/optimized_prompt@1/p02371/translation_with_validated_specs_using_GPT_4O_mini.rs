use std::collections::VecDeque;
use std::io::{self, BufRead};

const INF: i32 = 1_000_000_000;

fn max(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

fn dfs(
    v: usize,
    e: usize,
    s: usize,
    d: i32,
    graph: &Vec<Vec<(usize, i32)>>,
    visited: &mut Vec<bool>,
    costs: &mut Vec<i32>,
) -> i32 {
    let mut max_cost = d;
    visited[s] = true;
    for &(to, cost) in &graph[s] {
        if visited[to] {
            continue;
        }
        costs[to] = costs[s] + cost;
        max_cost = max(max_cost, dfs(v, e, to, d + cost, graph, visited, costs));
    }
    max_cost
}

fn bfs(
    v: usize,
    e: usize,
    s: usize,
    graph: &Vec<Vec<(usize, i32)>>,
    costs: &mut Vec<i32>,
) -> i32 {
    let mut queue = VecDeque::new();
    let mut max_cost = 0;
    costs[s] = 0;
    queue.push_back(s);

    while let Some(current) = queue.pop_front() {
        for &(to, cost) in &graph[current] {
            if costs[to] > costs[current] + cost {
                costs[to] = costs[current] + cost;
                if max_cost < costs[to] {
                    max_cost = costs[to];
                }
                queue.push_back(to);
            }
        }
    }
    max_cost
}

fn tyokkei(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let mut graph = vec![Vec::new(); n];
    let mut costs = vec![INF; n];
    let mut visited = vec![false; n];

    for &(a, b, c) in edges {
        graph[a].push((b, c));
        graph[b].push((a, c)); // Undirected graph
    }

    costs[0] = 0;
    visited[0] = true;

    let max_cost = dfs(n, edges.len(), 0, 0, &graph, &mut visited, &mut costs);
    
    let mut g = 0;
    for i in 0..n {
        if max_cost == costs[i] {
            g = i;
        }
        costs[i] = INF; // Reset costs for the next DFS
        visited[i] = false; // Reset visited for the next DFS
    }

    costs[g] = 0;
    visited[g] = true;

    dfs(n, edges.len(), g, 0, &graph, &mut visited, &mut costs)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges = Vec::new();

    for _ in 0..n - 1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges.push((parts[0] as usize, parts[1] as usize, parts[2]));
    }

    println!("{}", tyokkei(n, &edges));
}

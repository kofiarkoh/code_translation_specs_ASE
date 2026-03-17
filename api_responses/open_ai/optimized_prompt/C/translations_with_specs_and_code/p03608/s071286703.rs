use std::io::{self, BufRead};

const MAX_NODES: usize = 201;
const MAX_LOCATIONS: usize = 9;
const INF: i32 = 1_000_000_000;

struct Graph {
    distance: [[i32; MAX_NODES]; MAX_NODES],
    locations: Vec<usize>,
    result: i32,
    used: [bool; MAX_LOCATIONS],
    num_locations: usize,
}

impl Graph {
    fn new(num_nodes: usize, num_locations: usize) -> Self {
        let mut distance = [[INF; MAX_NODES]; MAX_NODES];
        for i in 1..=num_nodes {
            distance[i][i] = 0; // Distance to self is zero
        }
        Self {
            distance,
            locations: Vec::with_capacity(num_locations),
            result: INF,
            used: [false; MAX_LOCATIONS],
            num_locations,
        }
    }

    fn dfs(&mut self, current: usize, last: Option<usize>, dist: i32) {
        if current == self.num_locations + 1 {
            if self.result > dist {
                self.result = dist;
            }
            return;
        }

        for i in 1..=self.num_locations {
            if !self.used[i] {
                self.used[i] = true;
                if last.is_none() {
                    self.dfs(current + 1, Some(i), 0);
                } else {
                    let last_index = last.unwrap();
                    self.dfs(current + 1, Some(i), dist + self.distance[self.locations[last_index]][self.locations[i]]);
                }
                self.used[i] = false;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let r: usize = parts.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n, r);

    for i in 1..=r {
        let location: usize = lines.next().unwrap().unwrap().parse().unwrap();
        graph.locations.push(location);
    }

    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let a: usize = edge_parts.next().unwrap().parse().unwrap();
        let b: usize = edge_parts.next().unwrap().parse().unwrap();
        let c: i32 = edge_parts.next().unwrap().parse().unwrap();
        if graph.distance[a][b] > c {
            graph.distance[a][b] = c;
            graph.distance[b][a] = c; // Undirected graph
        }
    }

    // Floyd-Warshall algorithm to find shortest paths
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if graph.distance[i][j] > graph.distance[i][k] + graph.distance[k][j] {
                    graph.distance[i][j] = graph.distance[i][k] + graph.distance[k][j];
                }
            }
        }
    }

    graph.dfs(1, None, 0);

    println!("{}", graph.result);
}

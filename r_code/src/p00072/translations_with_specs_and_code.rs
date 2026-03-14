use std::io::{self, BufRead};

const INF: i32 = 1_000_000;
const MAX_VERTICES: usize = 100;

struct Graph {
    cost: [[i32; MAX_VERTICES]; MAX_VERTICES],
    mincost: [i32; MAX_VERTICES],
    used: [bool; MAX_VERTICES],
    v: usize,
}

impl Graph {
    fn new(v: usize) -> Self {
        let mut cost = [[INF; MAX_VERTICES]; MAX_VERTICES];
        for i in 0..v {
            cost[i][i] = 0; // No cost to reach itself
        }
        Self {
            cost,
            mincost: [INF; MAX_VERTICES],
            used: [false; MAX_VERTICES],
            v,
        }
    }

    fn min(a: i32, b: i32) -> i32 {
        if a > b { b } else { a }
    }

    fn prim(&mut self) -> i32 {
        self.mincost[0] = 0;
        let mut res = 0;

        loop {
            let mut v = usize::MAX;
            for u in 0..self.v {
                if !self.used[u] && (v == usize::MAX || self.mincost[u] < self.mincost[v]) {
                    v = u;
                }
            }

            if v == usize::MAX {
                break;
            }
            self.used[v] = true;
            res += self.mincost[v];

            for u in 0..self.v {
                self.mincost[u] = Self::min(self.mincost[u], self.cost[v][u]);
            }
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let v: usize = line.trim().parse().unwrap();
        if v == 0 {
            break;
        }
        let num: usize = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();

        let mut graph = Graph::new(v);

        for _ in 0..num {
            let edge_line = reader.lines().next().unwrap().unwrap();
            let parts: Vec<&str> = edge_line.split(',').collect();
            let from: usize = parts[0].trim().parse().unwrap();
            let to: usize = parts[1].trim().parse().unwrap();
            let dist: i32 = parts[2].trim().parse().unwrap();
            graph.cost[from][to] = graph.cost[to][from] = (dist / 100) - 1;
        }

        println!("{}", graph.prim());
    }
}

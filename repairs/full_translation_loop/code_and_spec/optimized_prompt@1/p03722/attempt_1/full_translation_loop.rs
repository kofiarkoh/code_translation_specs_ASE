use std::io::{self, BufRead};

const SUP: i64 = 1 << 60;

#[derive(Debug)]
struct Edge {
    t: usize,
    h: usize,
    cost: i64,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of nodes and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize edges
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let w: usize = parts.next().unwrap().parse().unwrap();
        let c: i64 = parts.next().unwrap().parse().unwrap();
        edges.push(Edge { t: u, h: w, cost: -c });
    }

    // Initialize minimum costs
    let mut min = vec![SUP; n + 1];
    min[1] = 0;

    // Bellman-Ford algorithm to find minimum costs
    for _ in 1..n {
        for edge in &edges {
            if min[edge.t] < SUP && min[edge.h] > min[edge.t] + edge.cost {
                min[edge.h] = min[edge.t] + edge.cost;
            }
        }
    }

    let ans = min[n];

    // Check for negative cycles affecting the path to the specified node
    for _ in 1..=n {
        for edge in &edges {
            if min[edge.t] < SUP && min[edge.h] > min[edge.t] + edge.cost {
                min[edge.h] = min[edge.t] + edge.cost;
            }
        }
    }

    // Output the result
    if min[n] == ans {
        println!("{}", -ans);
    } else {
        println!("inf");
    }
}

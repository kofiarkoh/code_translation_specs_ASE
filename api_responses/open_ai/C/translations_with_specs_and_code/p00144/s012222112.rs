use std::io::{self, BufRead};

const MAX_NODES: usize = 100;
const INF: i32 = 100_000;

fn dijkstra(graph: &[[i32; MAX_NODES]; MAX_NODES], n: usize, start: usize, goal: usize) -> i32 {
    let mut d = [INF; MAX_NODES];
    let mut f = [false; MAX_NODES];

    d[start] = 1; // Start distance is set to 1 as per the C code

    loop {
        let mut minimum = INF;
        let mut p = 0;

        for i in 0..n {
            if d[i] < minimum && !f[i] {
                minimum = d[i];
                p = i;
            }
        }

        if minimum == INF {
            break; // No more reachable nodes
        }

        f[p] = true; // Mark node as processed

        for i in 0..n {
            if d[p] + graph[p][i] < d[i] {
                d[i] = d[p] + graph[p][i];
            }
        }
    }

    d[goal]
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut graph = [[INF; MAX_NODES]; MAX_NODES];

    for i in 0..n {
        let edge_info: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let r1 = (edge_info[0] - 1) as usize; // Convert to zero-based index
        let k1 = edge_info[1];

        for j in 0..k1 {
            let temp = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
            graph[r1][(temp - 1) as usize] = 1; // Set edge weight to 1
        }
    }

    let p: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..p {
        let query: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let s1 = (query[0] - 1) as usize; // Convert to zero-based index
        let d1 = (query[1] - 1) as usize; // Convert to zero-based index
        let v1 = query[2];

        let temp = dijkstra(&graph, n, s1, d1);
        if v1 - temp < 0 {
            println!("NA");
        } else {
            println!("{}", temp);
        }
    }
}

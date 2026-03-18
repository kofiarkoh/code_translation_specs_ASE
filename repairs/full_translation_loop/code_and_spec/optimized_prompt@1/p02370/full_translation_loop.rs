use std::collections::VecDeque;
use std::io::{self, BufRead};

const MAX_V: usize = 10002;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of vertices and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let v: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    let mut to: Vec<Vec<usize>> = vec![Vec::new(); v];
    let mut hi: Vec<usize> = vec![0; v];
    let mut count: Vec<usize> = vec![0; v];

    // Read edges
    let mut edges = Vec::with_capacity(e);
    for _ in 0..e {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s: usize = parts.next().unwrap().parse().unwrap();
        let t: usize = parts.next().unwrap().parse().unwrap();
        edges.push((s, t));
        hi[s] += 1;
    }

    // Allocate adjacency list
    for (s, t) in edges {
        to[s].push(t);
        count[t] += 1;
    }

    // Perform topological sort
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..v {
        if count[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(i) = q.pop_front() {
        println!("{}", i);
        for &k in &to[i] {
            if count[k] > 0 {
                count[k] -= 1;
                if count[k] == 0 {
                    q.push_back(k);
                }
            }
        }
    }
}

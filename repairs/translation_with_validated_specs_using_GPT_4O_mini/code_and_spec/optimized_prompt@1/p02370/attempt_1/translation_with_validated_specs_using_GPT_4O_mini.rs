use std::collections::VecDeque;
use std::io::{self, BufRead};

const MAX_V: usize = 10002;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let v: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    let mut to: Vec<Vec<usize>> = vec![Vec::new(); v];
    let mut hi: Vec<usize> = vec![0; v];
    let mut count: Vec<usize> = vec![0; v];

    let mut memo: Vec<usize> = Vec::with_capacity(e * 2);
    for _ in 0..e {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let s: usize = edge_parts.next().unwrap().parse().unwrap();
        let t: usize = edge_parts.next().unwrap().parse().unwrap();
        memo.push(s);
        memo.push(t);
        hi[s] += 1;
    }

    for i in 0..v {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
        }
    }

    hi.fill(0); // Reset hi for adjacency list population
    let mut j = 0;
    while j < memo.len() {
        let s = memo[j];
        let t = memo[j + 1];
        to[s][hi[s]] = t;
        hi[s] += 1;
        count[t] += 1;
        j += 2;
    }

    topological_sort(v, &to, &count);
}

fn topological_sort(v: usize, to: &Vec<Vec<usize>>, count: &Vec<usize>) {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut in_degree = count.clone();

    for i in 0..v {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(i) = queue.pop_front() {
        println!("{}", i);
        for &k in &to[i] {
            if in_degree[k] > 0 {
                in_degree[k] -= 1;
                if in_degree[k] == 0 {
                    queue.push_back(k);
                }
            }
        }
    }
}

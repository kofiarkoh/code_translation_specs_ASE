// AOJ 2251 Merry Christmas
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX: usize = 2002;
const INF: i32 = 0x1010;

fn bpm(u: usize, hi: &[usize], to: &[Vec<usize>], match_: &mut [i32], seen: &mut [bool]) -> bool {
    for &v in &to[u][..hi[u]] {
        if seen[v] {
            continue;
        }
        seen[v] = true;
        if match_[v] < 0 || bpm(match_[v] as usize, hi, to, match_, seen) {
            match_[u] = v as i32;
            match_[v] = u as i32;
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().filter_map(Result::ok);
    
    let mut dist = vec![vec![INF; 102]; 102];
    let mut p = vec![0; 1002];
    let mut t = vec![0; 1002];
    let mut hi = vec![0; MAX];
    let mut to = vec![vec![]; MAX];
    let mut match_ = vec![-1; MAX];
    
    while let Some(line) = input.next() {
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        
        let m: usize = input.next().unwrap().trim().parse().unwrap();
        let l: usize = input.next().unwrap().trim().parse().unwrap();
        
        // Initialize distance matrix
        for k in 0..n {
            dist[k][k] = 0;
        }
        
        for _ in 0..m {
            let connection: Vec<usize> = input.next().unwrap().trim().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            let u = connection[0];
            let v = connection[1];
            let d: i32 = connection[2] as i32; // Convert to i32
            dist[u][v] = d;
            dist[v][u] = d;
        }
        
        // Floyd-Warshall algorithm to find shortest paths
        for k in 0..n {
            for u in 0..n {
                if dist[u][k] != INF {
                    for v in 0..n {
                        if dist[k][v] != INF {
                            let d = dist[u][k] + dist[k][v];
                            if dist[u][v] > d {
                                dist[u][v] = d;
                            }
                        }
                    }
                }
            }
        }
        
        // Prepare tasks
        for k in 0..l {
            let task: Vec<usize> = input.next().unwrap().trim().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            p[k] = task[0];
            t[k] = task[1] as i32; // Convert to i32
        }
        
        // Build bipartite graph
        for u in 0..l {
            for v in 0..l {
                if u != v {
                    if dist[p[u]][p[v]] <= t[v] - t[u] {
                        to[u].push(v + l);
                        hi[u] += 1;
                    }
                }
            }
        }
        
        let mut k = l;
        for u in 0..l {
            let mut seen = vec![false; MAX];
            if bpm(u, &hi, &to, &mut match_, &mut seen) {
                k -= 1;
            }
        }
        
        println!("{}", k);
    }
}

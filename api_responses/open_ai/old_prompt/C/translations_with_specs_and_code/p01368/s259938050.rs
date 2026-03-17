// AOJ 2251 Merry Christmas
// 2018.3.6 bal4u

use std::io::{self, BufRead};

const MAX: usize = 2002;
const INF: i32 = 0x1010;

struct Graph {
    dist: [[i32; 102]; 102],
    hi: [usize; MAX],
    to: [[usize; MAX]; MAX],
    match_: [i32; MAX],
    seen: [bool; MAX],
}

impl Graph {
    fn new() -> Self {
        Self {
            dist: [[INF; 102]; 102],
            hi: [0; MAX],
            to: [[0; MAX]; MAX],
            match_: [-1; MAX],
            seen: [false; MAX],
        }
    }

    fn bpm(&mut self, u: usize) -> bool {
        for i in 0..self.hi[u] {
            let v = self.to[u][i];
            if self.seen[v] {
                continue;
            }
            self.seen[v] = true;
            if self.match_[v] < 0 || self.bpm(self.match_[v] as usize) {
                self.match_[u] = v as i32;
                self.match_[v] = u as i32;
                return true;
            }
        }
        false
    }

    fn floyd_warshall(&mut self, n: usize) {
        for k in 0..n {
            for u in 0..n {
                if self.dist[u][k] != INF {
                    for v in 0..n {
                        if self.dist[k][v] != INF {
                            let d = self.dist[u][k] + self.dist[k][v];
                            if self.dist[u][v] > d {
                                self.dist[u][v] = d;
                            }
                        }
                    }
                }
            }
        }
    }

    fn process_tasks(&mut self, p: &[usize], t: &[i32], l: usize) -> i32 {
        for u in 0..l {
            for v in 0..l {
                if u != v {
                    if self.dist[p[u]][p[v]] <= t[v] - t[u] {
                        self.to[u][self.hi[u]] = v + l;
                        self.hi[u] += 1;
                    }
                }
            }
        }

        self.match_.fill(-1);
        let mut k = l as i32;
        for u in 0..l {
            self.seen.fill(false);
            if self.bpm(u) {
                k -= 1;
            }
        }
        k
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().filter_map(Result::ok);
    
    let mut graph = Graph::new();

    while let Some(line) = input.next() {
        let n: usize = line.trim().parse().unwrap();
        let m: usize = input.next().unwrap().trim().parse().unwrap();
        let l: usize = input.next().unwrap().trim().parse().unwrap();

        // Initialize distances
        for i in 0..n {
            graph.dist[i][i] = 0;
        }
        for _ in 0..m {
            let edge: Vec<usize> = input.next().unwrap().trim().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            let u = edge[0];
            let v = edge[1];
            let d = edge[2] as i32;
            graph.dist[u][v] = d;
            graph.dist[v][u] = d;
        }

        // Run Floyd-Warshall
        graph.floyd_warshall(n);

        let mut p = vec![0; l];
        let mut t = vec![0; l];
        for k in 0..l {
            let task: Vec<usize> = input.next().unwrap().trim().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            p[k] = task[0];
            t[k] = task[1] as i32;
        }

        let result = graph.process_tasks(&p, &t, l);
        println!("{}", result);
    }
}

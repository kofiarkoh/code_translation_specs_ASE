// AOJ 2251 Merry Christmas
// 2018.3.6 bal4u

use std::io::{self, BufRead};

const MAX: usize = 2002;
const INF: i32 = 0x1010;

struct Graph {
    hi: [usize; MAX],
    to: [[usize; MAX]; MAX],
    dist: [[i32; 102]; 102],
    match_: [i32; MAX],
    seen: [bool; MAX],
}

impl Graph {
    fn new() -> Self {
        Graph {
            hi: [0; MAX],
            to: [[0; MAX]; MAX],
            dist: [[INF; 102]; 102],
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
}

fn read_int() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut graph = Graph::new();
    let mut line = String::new();

    while let Ok(n) = read_int() {
        let m = read_int();
        let l = read_int();

        // Initialize distances
        for i in 0..n {
            graph.dist[i][i] = 0;
        }
        for _ in 0..m {
            let u = read_int() as usize;
            let v = read_int() as usize;
            let d = read_int();
            graph.dist[u][v] = d;
            graph.dist[v][u] = d;
        }

        graph.floyd_warshall(n);

        graph.hi = [0; MAX];
        let mut p = vec![0; l as usize];
        let mut t = vec![0; l as usize];

        for k in 0..l {
            p[k as usize] = read_int();
            t[k as usize] = read_int();
        }

        for u in 0..l {
            for v in 0..l {
                if u != v {
                    if graph.dist[p[u as usize] as usize][p[v as usize] as usize] <= t[v as usize] - t[u as usize] {
                        graph.to[u as usize][graph.hi[u as usize]] = v + l;
                        graph.hi[u as usize] += 1;
                    }
                }
            }
        }

        graph.match_ = [-1; MAX];
        let mut k = l;

        for u in 0..l {
            graph.seen = [false; MAX];
            if graph.bpm(u as usize) {
                k -= 1;
            }
        }

        println!("{}", k);
    }
}

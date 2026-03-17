// AOJ 2222 Alien's Counting
// 2018.4.25 bal4u

use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX_V: usize = 1002;
const MOD: i64 = 1_000_000_007;

struct Graph {
    to: Vec<Vec<usize>>,
    r_to: Vec<Vec<usize>>,
    hi: Vec<usize>,
    r_hi: Vec<usize>,
    lim: Vec<usize>,
    r_lim: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            to: vec![Vec::new(); n],
            r_to: vec![Vec::new(); n],
            hi: vec![0; n],
            r_hi: vec![0; n],
            lim: vec![0; n],
            r_lim: vec![0; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.check_lim(from);
        self.check_r_lim(to);
        self.to[from].push(to);
        self.hi[from] += 1;
        self.r_to[to].push(from);
        self.r_hi[to] += 1;
    }

    fn check_lim(&mut self, a: usize) {
        if self.hi[a] < self.lim[a] {
            return;
        }
        if self.lim[a] == 0 {
            self.lim[a] = 8;
            self.hi[a] = 0;
        } else if self.hi[a] == self.lim[a] {
            let k = self.lim[a].min(256);
            self.lim[a] += k;
        }
    }

    fn check_r_lim(&mut self, a: usize) {
        if self.r_hi[a] < self.r_lim[a] {
            return;
        }
        if self.r_lim[a] == 0 {
            self.r_lim[a] = 8;
            self.r_hi[a] = 0;
        } else if self.r_hi[a] == self.r_lim[a] {
            let k = self.r_lim[a].min(256);
            self.r_lim[a] += k;
        }
    }

    fn scc(&mut self) -> usize {
        let mut used = vec![false; self.to.len()];
        let mut vs = Vec::new();
        let mut vsz = 0;

        for v in 0..self.to.len() {
            if !used[v] {
                self.dfs(v, &mut used, &mut vs, &mut vsz);
            }
        }

        used.fill(false);
        let mut k = 0;
        for &v in vs.iter().rev() {
            if !used[v] {
                self.rdfs(v, k, &mut used);
                k += 1;
            }
        }
        k
    }

    fn dfs(&self, v: usize, used: &mut Vec<bool>, vs: &mut Vec<usize>, vsz: &mut usize) {
        used[v] = true;
        for &neighbor in &self.to[v] {
            if !used[neighbor] {
                self.dfs(neighbor, used, vs, vsz);
            }
        }
        vs.push(v);
        *vsz += 1;
    }

    fn rdfs(&self, v: usize, k: usize, used: &mut Vec<bool>) {
        used[v] = true;
        for &neighbor in &self.r_to[v] {
            if !used[neighbor] {
                self.rdfs(neighbor, k, used);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n);
    
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_iter = edge_line.split_whitespace();
        let s: usize = edge_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let d: usize = edge_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        graph.add_edge(s, d);
    }

    let k = graph.scc();
    let mut dp = vec![0; k];
    let mut nx = vec![-1; k];

    for i in 0..n {
        if graph.hi[i] > 0 {
            nx[graph.to[i][0]] = graph.to[i][0] as i64; // Assuming cmp[i] is the index of the SCC
        }
    }

    let mut ans = 1;
    for i in 0..k {
        if dp[i] > 0 {
            dp[i] += 1;
        } else {
            dp[i] = 2;
        }
        if nx[i] < 0 || nx[i] == i as i64 {
            ans = (ans * dp[i] as i64) % MOD;
        } else if dp[nx[i] as usize] > 0 {
            dp[nx[i] as usize] = (dp[nx[i] as usize] as i64 * dp[i] as i64) % MOD;
        } else {
            dp[nx[i] as usize] = dp[i];
        }
    }
    println!("{}", ans);
}

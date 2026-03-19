// AOJ 2222 Alien's Counting
// 2018.4.25 bal4u

use std::io::{self, Read};

const MAX_V: usize = 1002;

struct Graph {
    to: Vec<Vec<usize>>,
    hi: Vec<usize>,
    lim: Vec<usize>,
    r_to: Vec<Vec<usize>>,
    r_hi: Vec<usize>,
    r_lim: Vec<usize>,
    vs: Vec<usize>,
    vsz: usize,
    used: Vec<bool>,
    cmp: Vec<usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            to: vec![Vec::new(); MAX_V],
            hi: vec![0; MAX_V],
            lim: vec![0; MAX_V],
            r_to: vec![Vec::new(); MAX_V],
            r_hi: vec![0; MAX_V],
            r_lim: vec![0; MAX_V],
            vs: Vec::new(),
            vsz: 0,
            used: vec![false; MAX_V],
            cmp: vec![0; MAX_V],
        }
    }

    fn check_lim(&mut self, a: usize) {
        if self.hi[a] < self.lim[a] {
            return;
        }
        if self.lim[a] == 0 {
            self.lim[a] = 8;
            self.hi[a] = 0;
            self.to[a].resize(self.lim[a], 0);
        } else if self.hi[a] == self.lim[a] {
            let k = self.lim[a].min(256);
            self.lim[a] += k;
            self.to[a].resize(self.lim[a], 0);
        }
    }

    fn check_r_lim(&mut self, a: usize) {
        if self.r_hi[a] < self.r_lim[a] {
            return;
        }
        if self.r_lim[a] == 0 {
            self.r_lim[a] = 8;
            self.r_hi[a] = 0;
            self.r_to[a].resize(self.r_lim[a], 0);
        } else if self.r_hi[a] == self.r_lim[a] {
            let k = self.r_lim[a].min(256);
            self.r_lim[a] += k;
            self.r_to[a].resize(self.r_lim[a], 0);
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.check_lim(from);
        self.check_r_lim(to);
        self.to[from][self.hi[from]] = to;
        self.hi[from] += 1;
        self.r_to[to][self.r_hi[to]] = from;
        self.r_hi[to] += 1;
    }

    fn dfs(&mut self, v: usize) {
        self.used[v] = true;
        let next_nodes: Vec<usize> = self.to[v][..self.hi[v]].iter().copied().collect();
        for next in next_nodes {
            if !self.used[next] {
                self.dfs(next);
            }
        }
        self.vs.push(v);
        self.vsz += 1;
    }

    fn rdfs(&mut self, v: usize, k: usize) {
        self.used[v] = true;
        self.cmp[v] = k;
        let next_nodes: Vec<usize> = self.r_to[v][..self.r_hi[v]].iter().copied().collect();
        for next in next_nodes {
            if !self.used[next] {
                self.rdfs(next, k);
            }
        }
    }

    fn scc(&mut self, v_count: usize) -> usize {
        self.used.fill(false);
        self.vsz = 0;
        for v in 0..v_count {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.used.fill(false);
        let mut k = 0;
        for &v in self.vs.iter().rev() {
            if !self.used[v] {
                self.rdfs(v, k);
                k += 1;
            }
        }
        k
    }
}

fn in_number() -> usize {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    iter.next().unwrap().parse().unwrap()
}

fn main() {
    const MOD: usize = 1_000_000_007;
    let n = in_number();
    let m = in_number();

    let mut graph = Graph::new();
    for _ in 0..m {
        let s = in_number() - 1;
        let d = in_number() - 1;
        graph.add_edge(s, d);
    }

    let k = graph.scc(n);
    let mut nx = vec![-1isize; k]; // Changed usize to isize
    let mut dp = vec![0; k];

    for i in 0..n {
        if graph.hi[i] > 0 {
            nx[graph.cmp[i]] = graph.cmp[graph.to[i][0]] as isize; // Cast to isize
        }
    }

    let mut ans: usize = 1;
    for i in 0..k {
        if dp[i] > 0 {
            dp[i] += 1;
        } else {
            dp[i] = 2;
        }
        if nx[i] < 0 || nx[i] == i as isize {
            ans = ans * dp[i] % MOD;
        } else if dp[nx[i] as usize] > 0 {
            dp[nx[i] as usize] = dp[nx[i] as usize] * dp[i] % MOD;
        } else {
            dp[nx[i] as usize] = dp[i];
        }
    }
    println!("{}", ans);
}

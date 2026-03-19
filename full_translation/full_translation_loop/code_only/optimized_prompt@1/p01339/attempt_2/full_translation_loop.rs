// AOJ 2222 Alien's Counting
// 2018.4.25 bal4u

use std::io::{self, BufRead};

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
        for &next in &self.to[v][..self.hi[v]] {
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
        for &next in &self.r_to[v][..self.r_hi[v]] {
            if !self.used[next] {
                self.rdfs(next, k);
            }
        }
    }

    fn scc(&mut self) -> usize {
        self.used.fill(false);
        self.vsz = 0;
        for v in 0..MAX_V {
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
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines().next().unwrap().unwrap();
    let mut n = 0;
    while let Some(c) = input.chars().next() {
        if c.is_digit(10) {
            n = n * 10 + c.to_digit(10).unwrap() as usize;
            input.remove(0);
        } else {
            break;
        }
    }
    n
}

const MOD: usize = 1_000_000_007;

fn main() {
    let mut graph = Graph::new();
    let n = in_number();
    let m = in_number();

    for _ in 0..m {
        let s = in_number() - 1;
        let d = in_number() - 1;
        graph.add_edge(s, d);
    }

    let k = graph.scc();
    let mut nx = vec![usize::MAX; k]; // Use usize::MAX to represent -1
    let mut dp = vec![0; k];

    for i in 0..n {
        if graph.hi[i] > 0 {
            nx[graph.cmp[i]] = graph.cmp[graph.to[i][0]];
        }
    }

    let mut ans: usize = 1;
    for i in 0..k {
        if dp[i] > 0 {
            dp[i] += 1;
        } else {
            dp[i] = 2;
        }
        if nx[i] == usize::MAX || nx[i] == i {
            ans = ans * dp[i] % MOD;
        } else if dp[nx[i]] > 0 {
            dp[nx[i]] = dp[nx[i]] * dp[i] % MOD;
        } else {
            dp[nx[i]] = dp[i];
        }
    }
    println!("{}", ans);
}

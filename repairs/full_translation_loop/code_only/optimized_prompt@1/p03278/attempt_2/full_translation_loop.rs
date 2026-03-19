use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MAX_N: usize = 5005;

fn calc(x: usize, pw: &[i64]) -> i64 {
    if x & 1 == 1 {
        0
    } else {
        pw[x >> 1]
    }
}

struct Graph {
    lst: Vec<i32>,
    to: Vec<i32>,
    pre: Vec<i32>,
    tot: usize,
    sz: Vec<usize>,
    dp_sz: Vec<usize>,
    dp: Vec<Vec<[i64; 2]>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            lst: vec![-1; n],
            to: vec![0; 10005],
            pre: vec![0; 10005],
            tot: 0,
            sz: vec![0; n],
            dp_sz: vec![0; n],
            dp: vec![vec![[0; 2]; MAX_N]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.to[self.tot] = v as i32;
        self.pre[self.tot] = self.lst[u];
        self.lst[u] = self.tot as i32;
        self.tot += 1;
    }

    fn init_dfs(&mut self, u: usize, fa: i32) {
        self.sz[u] = 1;
        let mut i = self.lst[u];
        while i != -1 {
            let v = self.to[i as usize] as usize;
            if v == fa as usize {
                i = self.pre[i as usize];
                continue;
            }
            self.init_dfs(v, u as i32);
            self.sz[u] += self.sz[v];
            i = self.pre[i as usize];
        }
    }

    fn dfs(&mut self, u: usize, fa: i32, pw: &[i64]) {
        self.dp[u][0][0] = 1;
        let mut i = self.lst[u];
        while i != -1 {
            let v = self.to[i as usize] as usize;
            if v == fa as usize {
                i = self.pre[i as usize];
                continue;
            }
            self.dfs(v, u as i32, pw);
            let mut tmp = [[0; 2]; MAX_N];
            for x in (0..=self.dp_sz[u]).rev() {
                for y in (0..=self.dp_sz[v]).rev() {
                    tmp[x + y][0] = (tmp[x + y][0] + self.dp[u][x][0] * self.dp[v][y][0]) % MOD;
                    tmp[x + y][0] = (tmp[x + y][0] + self.dp[u][x][1] * self.dp[v][y][1]) % MOD;
                    tmp[x + y][1] = (tmp[x + y][1] + self.dp[u][x][0] * self.dp[v][y][1]) % MOD;
                    tmp[x + y][1] = (tmp[x + y][1] + self.dp[u][x][1] * self.dp[v][y][0]) % MOD;
                }
            }
            self.dp[u].copy_from_slice(&tmp);
            self.dp_sz[u] += self.dp_sz[v];
            i = self.pre[i as usize];
        }
        if u > 0 {
            for x in (0..=self.dp_sz[u]).rev() {
                self.dp[u][self.sz[u]][0] = (self.dp[u][self.sz[u]][0] + self.dp[u][x][1] * calc(self.sz[u] - x, pw)) % MOD;
                self.dp[u][self.sz[u]][1] = (self.dp[u][self.sz[u]][1] + self.dp[u][x][0] * calc(self.sz[u] - x, pw)) % MOD;
            }
        }
        self.dp_sz[u] += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut pw = vec![1; MAX_N];
    
    for i in 1..=n {
        pw[i] = (pw[i - 1] * (i * 2 - 1) % MOD + MOD) % MOD; // Ensure non-negative
    }
    
    let mut graph = Graph::new(n);
    
    for _ in 1..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        graph.add_edge(u, v);
        graph.add_edge(v, u);
    }
    
    graph.init_dfs(0, -1);
    graph.dfs(0, -1, &pw);
    
    let mut ans = 0;
    for i in 0..=n {
        ans = (ans + graph.dp[0][i][0] * calc(n - i, &pw)) % MOD;
        ans = (ans - graph.dp[0][i][1] * calc(n - i, &pw)) % MOD;
    }
    
    println!("{}", (ans + MOD) % MOD);
}

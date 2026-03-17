use std::io::{self, BufRead};

const MAX_NODES: usize = 100010;
const MAX_EDGES: usize = 200010;
const INF: i32 = 1_000_000_000;

struct Graph {
    ta: Vec<i32>,
    nt: Vec<i32>,
    fr: Vec<i32>,
    to: Vec<i32>,
    co: Vec<i32>,
    C: Vec<i32>,
    f: Vec<i32>,
    q: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            ta: vec![-1; n],
            nt: vec![0; MAX_EDGES],
            fr: vec![0; MAX_EDGES],
            to: vec![0; MAX_EDGES],
            co: vec![0; MAX_EDGES],
            C: vec![INF; n],
            f: vec![0; n],
            q: vec![0; MAX_NODES],
        }
    }

    fn dfs(&mut self, v: usize, e: usize, s: usize, d: i32) -> i32 {
        let mut max = d;
        for mut i in self.ta[s] as usize.. {
            if i + 1 == 0 { break; } // End of list
            if self.f[self.to[i as usize] as usize] != 0 {
                continue;
            }
            self.f[self.to[i as usize] as usize] = 1;
            self.C[self.to[i as usize] as usize] = self.C[s] + self.co[i as usize];
            max = max.max(self.dfs(v, e, self.to[i as usize] as usize, d + self.co[i as usize]));
        }
        max
    }

    fn bfs(&mut self, v: usize, e: usize, s: usize) -> (i32, usize) {
        let mut max = 0;
        self.C[s] = 0;
        self.q[0] = s;
        let mut r = 1;
        let mut t = 0;

        while t < r {
            let current = self.q[t];
            for mut i in self.ta[current] as usize.. {
                if i + 1 == 0 { break; } // End of list
                if self.C[self.to[i as usize] as usize] > self.C[self.fr[i as usize] as usize] + self.co[i as usize] {
                    self.C[self.to[i as usize] as usize] = self.C[self.fr[i as usize] as usize] + self.co[i as usize];
                    if max < self.C[self.to[i as usize] as usize] {
                        max = self.C[self.to[i as usize] as usize];
                    }
                    self.q[r] = self.to[i as usize] as usize;
                    r += 1;
                }
            }
            t += 1;
        }
        (max, self.q[0]) // Return max and the first node
    }

    fn tyokkei(&mut self, v: usize, e: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32 {
        let mut max = 0;
        for i in 0..v {
            self.ta[i] = -1;
            self.C[i] = INF;
            self.f[i] = 0;
        }
        self.C[0] = 0;
        self.f[0] = 1;

        for i in 0..e {
            let idx = i as usize;
            self.nt[i] = self.ta[a[idx] as usize];
            self.fr[i] = a[idx];
            self.to[i] = b[idx];
            self.co[i] = c[idx];
            self.ta[a[idx] as usize] = i as i32;
            self.nt[i + e] = self.ta[b[idx] as usize];
            self.fr[i + e] = b[idx];
            self.to[i + e] = a[idx];
            self.co[i + e] = c[idx];
            self.ta[b[idx] as usize] = (i + e) as i32;
        }

        max = self.dfs(v, e * 2, 0, 0);
        let mut g = 0;
        for i in 0..v {
            if max == self.C[i] {
                g = i;
            }
            self.C[i] = INF;
            self.f[i] = 0;
        }
        self.C[g] = 0;
        self.f[g] = 1;
        self.dfs(v, e * 2, g, 0)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n - 1];
    let mut b = vec![0; n - 1];
    let mut c = vec![0; n - 1];

    for i in 0..n - 1 {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0];
        b[i] = nums[1];
        c[i] = nums[2];
    }

    let mut graph = Graph::new(n);
    let result = graph.tyokkei(n, n - 1, &a, &b, &c);
    println!("{}", result);
}

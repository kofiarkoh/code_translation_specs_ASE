use std::io::{self, BufRead};

const MAX_VERTICES: usize = 100010;
const MAX_EDGES: usize = 200010;
const INF: i32 = 1000000000;

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
    fn new(vertices: usize) -> Self {
        Self {
            ta: vec![-1; vertices],
            nt: vec![0; MAX_EDGES],
            fr: vec![0; MAX_EDGES],
            to: vec![0; MAX_EDGES],
            co: vec![0; MAX_EDGES],
            C: vec![INF; vertices],
            f: vec![0; vertices],
            q: vec![0; MAX_VERTICES],
        }
    }

    fn dfs(&mut self, v: usize, e: usize, s: usize, d: i32) -> i32 {
        let mut max = d;
        for i in (self.ta[s] as usize..).take_while(|&i| i < e) {
            if self.f[self.to[i] as usize] != 0 {
                continue;
            }
            self.f[self.to[i] as usize] = 1;
            self.C[self.to[i] as usize] = self.C[s] + self.co[i];
            max = max.max(self.dfs(v, e, self.to[i] as usize, d + self.co[i]));
        }
        max
    }

    fn bfs(&mut self, v: usize, e: usize, s: usize) -> (i32, usize) {
        let mut max = 0;
        let mut t = 0;
        self.C[s] = 0;
        self.f[s] = 1;
        self.q[t] = s;

        let mut r = 1;
        while t < r {
            let current = self.q[t];
            t += 1;

            for i in (self.ta[current] as usize..).take_while(|&i| i < e) {
                if self.C[self.to[i] as usize] > self.C[self.fr[i] as usize] + self.co[i] {
                    self.C[self.to[i] as usize] = self.C[self.fr[i] as usize] + self.co[i];
                    if max < self.C[self.to[i] as usize] {
                        max = self.C[self.to[i] as usize];
                    }
                    self.q[r] = self.to[i] as usize;
                    r += 1;
                }
            }
        }
        (max, self.q[t - 1])
    }

    fn tyokkei(&mut self, v: usize, e: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32 {
        let mut max = 0;
        let mut g = 0;

        for i in 0..v {
            self.ta[i] = -1;
            self.C[i] = INF;
            self.f[i] = 0;
        }

        self.C[0] = 0;
        self.f[0] = 1;

        for i in 0..e {
            self.nt[i] = self.ta[a[i] as usize] as i32;
            self.fr[i] = a[i];
            self.to[i] = b[i];
            self.co[i] = c[i];
            self.ta[a[i] as usize] = i as i32;
            self.nt[i + e] = self.ta[b[i] as usize] as i32;
            self.fr[i + e] = b[i];
            self.to[i + e] = a[i];
            self.co[i + e] = c[i];
            self.ta[b[i] as usize] = (i + e) as i32;
        }

        max = self.dfs(v, e * 2, 0, 0);
        for i in 0..v {
            if max == self.C[i] {
                g = i;
            }
            self.C[i] = INF;
            self.f[i] = 0;
        }
        self.C[g as usize] = 0;
        self.f[g as usize] = 1;

        self.dfs(v, e * 2, g as usize, 0)
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

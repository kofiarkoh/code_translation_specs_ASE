use std::io::{self, BufRead};

const INF: i32 = 1_000_000_000;

fn max(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

struct Graph {
    ta: Vec<i32>,
    nt: Vec<i32>,
    fr: Vec<i32>,
    to: Vec<i32>,
    co: Vec<i32>,
    C: Vec<i32>,
    f: Vec<bool>,
}

impl Graph {
    fn new(vertices: usize, edges: usize) -> Self {
        Self {
            ta: vec![-1; vertices],
            nt: vec![0; edges * 2],
            fr: vec![0; edges * 2],
            to: vec![0; edges * 2],
            co: vec![0; edges * 2],
            C: vec![INF; vertices],
            f: vec![false; vertices],
        }
    }

    fn dfs(&mut self, v: usize, e: usize, s: usize, d: i32) -> i32 {
        let mut max_cost = d; // Renamed to avoid shadowing
        for i in (self.ta[s] as usize..).take_while(|&i| i < e) {
            if self.f[self.to[i] as usize] {
                continue;
            }
            self.f[self.to[i] as usize] = true;
            self.C[self.to[i] as usize] = self.C[s] + self.co[i];
            max_cost = max(max_cost, self.dfs(v, e, self.to[i] as usize, d + self.co[i]));
        }
        max_cost
    }

    fn bfs(&mut self, v: usize, e: usize, s: usize) -> (i32, usize) {
        let mut max = 0;
        let mut queue = vec![0; v];
        let mut r = 1;
        let mut t = 0;

        self.C[s] = 0;
        self.f[s] = true;

        while t < r {
            let current = queue[t];
            t += 1;

            for i in (self.ta[current] as usize..).take_while(|&i| i < e) {
                if self.C[self.to[i] as usize] > self.C[self.fr[i] as usize] + self.co[i] {
                    self.C[self.to[i] as usize] = self.C[self.fr[i] as usize] + self.co[i];
                    if max < self.C[self.to[i] as usize] {
                        max = self.C[self.to[i] as usize];
                    }
                    queue[r] = self.to[i] as usize; // Ensure correct type
                    r += 1;
                }
            }
        }
        (max, queue[t - 1] as usize)
    }

    fn compute_max_cost(&mut self, v: usize, e: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32 {
        let mut max = 0;
        let mut g = 0;

        for i in 0..v {
            self.ta[i] = -1;
            self.C[i] = INF;
            self.f[i] = false;
        }

        self.C[0] = 0;
        self.f[0] = true;

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
            self.f[i] = false;
        }
        self.C[g] = 0;
        self.f[g] = true;

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

    let mut graph = Graph::new(n, n - 1);
    let result = graph.compute_max_cost(n, n - 1, &a, &b, &c);
    println!("{}", result);
}

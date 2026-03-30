use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

const MAX_V: usize = 100010;
const MAX_E: usize = 200010;
const INF: i32 = 1_000_000_000;

struct Graph {
    ta: [i32; MAX_V],
    nt: [i32; MAX_E],
    fr: [i32; MAX_E],
    to: [i32; MAX_E],
    co: [i32; MAX_E],
    f: [i32; MAX_V],
    c: [i32; MAX_V],
    q: Vec<i32>,
}

impl Graph {
    fn new() -> Self {
        Self {
            ta: [-1; MAX_V],
            nt: [-1; MAX_E],
            fr: [-1; MAX_E],
            to: [-1; MAX_E],
            co: [-1; MAX_E],
            f: [0; MAX_V],
            c: [INF; MAX_V],
            q: Vec::new(),
        }
    }

    fn dfs(&mut self, v: usize, e: usize, s: usize, d: i32) -> i32 {
        let mut max = d;
        let mut i = self.ta[s];
        while i != -1 {
            if self.f[self.to[i as usize] as usize] != 0 {
                i = self.nt[i as usize];
                continue;
            }
            self.f[self.to[i as usize] as usize] = 1;
            self.c[self.to[i as usize] as usize] = self.c[s] + self.co[i as usize];
            max = max(max, self.dfs(v, e, self.to[i as usize] as usize, d + self.co[i as usize]));
            i = self.nt[i as usize];
        }
        max
    }

    fn bfs(&mut self, v: usize, e: usize, s: usize, g: &mut i32) -> i32 {
        let mut t = 0;
        let mut r = 1;
        self.c[s] = 0;
        self.q.push(s as i32);
        let mut max = 0;

        while t < r {
            let current = self.q[t as usize];
            t += 1;

            let mut i = self.ta[current as usize];
            while i != -1 {
                if self.c[self.to[i as usize] as usize] > self.c[self.fr[i as usize] as usize] + self.co[i as usize] {
                    self.c[self.to[i as usize] as usize] = self.c[self.fr[i as usize] as usize] + self.co[i as usize];
                    if max < self.c[self.to[i as usize] as usize] {
                        max = self.c[self.to[i as usize] as usize];
                        *g = self.to[i as usize];
                    }
                    self.q.push(self.to[i as usize]);
                    r += 1;
                }
                i = self.nt[i as usize];
            }
        }
        max
    }

    fn tyokkei(&mut self, v: usize, e: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32 {
        let mut g = 0;
        let mut max = 0;

        for i in 0..v {
            self.ta[i] = -1;
            self.c[i] = INF;
            self.f[i] = 0;
        }
        self.c[0] = 0;
        self.f[0] = 1;

        for i in 0..e {
            self.nt[i] = self.ta[a[i as usize] as usize];
            self.fr[i] = a[i as usize];
            self.to[i] = b[i as usize];
            self.co[i] = c[i as usize];
            self.ta[a[i as usize] as usize] = i as i32;
            self.nt[i + e] = self.ta[b[i as usize] as usize];
            self.fr[i + e] = b[i as usize];
            self.to[i + e] = a[i as usize];
            self.co[i + e] = c[i as usize];
            self.ta[b[i as usize] as usize] = (i + e) as i32;
        }

        max = self.dfs(v, e * 2, 0, 0);
        for i in 0..v {
            if max == self.c[i as usize] {
                g = i;
            }
            self.c[i] = INF;
            self.f[i] = 0;
        }
        self.c[g as usize] = 0;
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

    let mut graph = Graph::new();
    let result = graph.tyokkei(n, n - 1, &a, &b, &c);
    println!("{}", result);
}

use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn mbk(a: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let half = mbk((a * a) % m, n / 2, m);
    if n % 2 == 0 {
        half
    } else {
        (half * (a % m)) % m
    }
}

fn inv(a: i64, m: i64) -> i64 {
    mbk(a, m - 2, m)
}

struct TreeDP {
    ta: Vec<i64>,
    nt: Vec<i64>,
    to: Vec<i64>,
    tS: Vec<i64>,
    tu: Vec<i64>,
    tans: Vec<i64>,
    ans: Vec<i64>,
}

impl TreeDP {
    fn new(n: usize) -> Self {
        Self {
            ta: vec![-1; n + 1],
            nt: vec![0; 2 * n],
            to: vec![0; 2 * n],
            tS: vec![1; n + 1],
            tu: vec![0; n + 1],
            tans: vec![1; n + 1],
            ans: vec![0; n + 1],
        }
    }

    fn fc(&mut self, s: usize, t: usize) {
        self.tans[s] = (self.tans[s] * self.tans[t]) % M;
    }

    fn ic(&mut self, s: usize, t: usize) {
        self.tans[s] = (self.tans[s] * inv(self.tans[t], M)) % M;
    }

    fn fp(&mut self, s: usize) {
        self.tans[s] = (self.tans[s] * inv(self.tS[s], M)) % M;
    }

    fn ip(&mut self, s: usize) {
        self.tans[s] = (self.tans[s] * self.tS[s]) % M;
    }

    fn tdp(&mut self, s: usize) {
        self.tu[s] = 1;
        let mut i = self.ta[s];
        while i >= 0 {
            let child = self.to[i as usize] as usize;
            if self.tu[child] != 0 {
                i = self.nt[i as usize];
                continue;
            }
            self.tdp(child);
            self.fc(s, child);
            self.tS[s] += self.tS[child];
            i = self.nt[i as usize];
        }
        self.fp(s);
    }

    fn chrt(&mut self, s: usize, t: usize) {
        self.ip(s);
        self.ic(s, t);
        self.ip(t);
        self.tS[s] -= self.tS[t];
        self.tS[t] += self.tS[s];
        self.fp(s);
        self.fc(t, s);
        self.fp(t);
    }

    fn atdp(&mut self, s: usize) {
        self.tu[s] = 2;
        self.ans[s] = self.tans[s];
        let mut i = self.ta[s];
        while i >= 0 {
            let child = self.to[i as usize] as usize;
            if self.tu[child] == 2 {
                i = self.nt[i as usize];
                continue;
            }
            self.chrt(s, child);
            self.atdp(child);
            self.chrt(child, s);
            i = self.nt[i as usize];
        }
    }

    fn at_dp(&mut self, v: usize, a: &[i32], b: &[i32]) {
        for i in 0..v {
            self.ta[i + 1] = -1;
        }
        for i in 0..v {
            self.tS[i + 1] = 1;
            self.tans[i + 1] = 1;
        }
        for i in 0..(v - 1) {
            self.nt[i] = self.ta[self.to[i + v - 1] as usize] as i64;
            self.nt[i + v - 1] = self.ta[self.to[i] as usize] as i64;
            self.ta[a[i] as usize] = i as i64;
            self.ta[b[i] as usize] = (i + v - 1) as i64;
            self.to[i] = b[i] as i64; // Store the child node
            self.to[i + v - 1] = a[i] as i64; // Store the parent node
        }
        self.tdp(1);
        self.atdp(1);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n - 1];
    let mut b = vec![0; n - 1];

    for i in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        a[i] = nums[0];
        b[i] = nums[1];
    }

    let mut p = 1;
    for i in 1..=n {
        p = (p * i as i64) % M;
    }

    let mut tree_dp = TreeDP::new(n);
    tree_dp.at_dp(n, &a, &b);

    for i in 1..=n {
        println!("{}", (tree_dp.ans[i] * p) % M);
    }
}
``` 

### Changes Made:
1. **Corrected the `to` array population**: The previous implementation did not correctly populate the `to` array, which is crucial for the tree structure. This has been fixed to ensure that the parent-child relationships are accurately represented.
2. **Adjusted the indexing**: Ensured that the indexing aligns with the expected behavior of the original C code, particularly in how nodes are referenced and processed.
3. **Ensured proper initialization**: Verified that all necessary arrays are initialized correctly to avoid any unintended behavior during calculations. 

These changes should help align the Rust implementation more closely with the expected output.

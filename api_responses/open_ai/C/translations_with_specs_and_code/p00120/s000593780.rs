use std::f64;
use std::io::{self, BufRead};

const INF: f64 = 100_000_000.0;
const EPS: f64 = 1e-5;

struct TSP {
    len: f64,
    cake: Vec<f64>,
    dp: Vec<Vec<f64>>,
    n: usize,
}

impl TSP {
    fn new() -> Self {
        Self {
            len: 0.0,
            cake: Vec::new(),
            dp: vec![vec![-INF; 12]; 1 << 12],
            n: 0,
        }
    }

    fn min(a: f64, b: f64) -> f64 {
        if b > a { a } else { b }
    }

    fn tsp(&mut self, bit: usize, now: usize) -> f64 {
        if self.dp[bit][now] >= 0.0 {
            return self.dp[bit][now];
        }

        if bit == (1 << self.n) - 1 && now == 0 {
            return self.dp[bit][now] = 0.0;
        }

        let mut res = INF;
        for i in 0..self.n {
            if (bit >> i) & 1 == 0 {
                let distance = ((self.cake[i] + self.cake[now]).powi(2) - (self.cake[i] - self.cake[now]).powi(2)).sqrt();
                res = Self::min(res, self.tsp(bit | (1 << i), i) + distance);
            }
        }

        self.dp[bit][now] = res;
        res
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let reader = stdin.lock();

        for line in reader.lines() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            self.len = parts.next().unwrap().parse().unwrap();
            self.cake.clear();
            self.n = 0;

            for part in parts {
                self.cake.push(part.parse().unwrap());
                self.n += 1;
            }

            // Initialize dp table
            for i in 0..(1 << 12) {
                for j in 0..12 {
                    self.dp[i][j] = -INF;
                }
            }

            if self.tsp(0, 0) < self.len + EPS {
                println!("OK");
            } else {
                println!("NA");
            }
        }
    }
}

fn main() {
    let mut tsp = TSP::new();
    tsp.run();
}

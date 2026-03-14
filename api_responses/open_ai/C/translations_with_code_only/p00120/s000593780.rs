use std::f64;
use std::io::{self, BufRead};

const SQ: fn(f64) -> f64 = |x| x * x;
const INF: f64 = 100000000.0;
const EPS: f64 = 1e-5;

static mut DP: [[f64; 12]; 1 << 12] = [[-1.0 * INF; 12]; 1 << 12];

fn min(a: f64, b: f64) -> f64 {
    if b > a { a } else { b }
}

fn tsp(bit: usize, now: usize, cake: &[f64], n: usize) -> f64 {
    unsafe {
        if DP[bit][now] >= 0.0 {
            return DP[bit][now];
        }
    }

    if bit == (1 << n) - 1 && now == 0 {
        unsafe {
            DP[bit][now] = 0.0;
        }
        return 0.0;
    }

    let mut res = INF;
    for i in 0..n {
        if (bit >> i) & 1 == 0 {
            let distance = (SQ(cake[i] + cake[now]) - SQ(cake[i] - cake[now])).sqrt();
            res = min(res, tsp(bit | (1 << i), i, cake, n) + distance);
        }
    }

    unsafe {
        DP[bit][now] = res;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let len: f64 = line.trim().parse().unwrap();
        let mut cake = Vec::new();
        let mut n = 0;

        loop {
            let mut input = String::new();
            if reader.read_line(&mut input).unwrap() == 0 {
                break;
            }
            let value: f64 = input.trim().parse().unwrap();
            cake.push(value);
            n += 1;

            if input.ends_with('\n') {
                break;
            }
        }

        unsafe {
            for i in 0..(1 << 12) {
                for j in 0..12 {
                    DP[i][j] = -1.0 * INF;
                }
            }
        }

        if tsp(0, 0, &cake, n) < len + EPS {
            println!("OK");
        } else {
            println!("NA");
        }
    }
}
